// @ts-nocheck
import 'jsr:@supabase/functions-js/edge-runtime.d.ts';
import OpenAI from 'npm:openai@4.56.0';
import { createClient } from 'npm:@supabase/supabase-js@2';
Deno.serve(async (req)=>{
  if (req.method !== 'POST') {
    return new Response('Method not allowed', {
      status: 405
    });
  }
  if (req.headers.get('content-type')?.includes('application/json') !== true) {
    return new Response('Expected application/json', {
      status: 400
    });
  }
  const supabase = createClient(Deno.env.get('SUPABASE_URL'), Deno.env.get('SUPABASE_SERVICE_ROLE_KEY'));
  let payload;
  try {
    payload = await req.json();
  } catch  {
    return new Response('Invalid JSON body', {
      status: 400
    });
  }
  const { room_id, limit = 50 } = payload || {};
  if (!room_id) return new Response('room_id is required', {
    status: 400
  });
  const limitNum = Math.max(1, Math.min(Number(limit) || 50, 200));
  // Authorization: derive user from JWT
  const authHeader = req.headers.get('authorization') || '';
  const token = authHeader.replace(/^Bearer\s+/i, '');
  if (!token) return new Response('Missing Bearer token', {
    status: 401
  });
  // Verify JWT by using Supabase auth endpoint via PostgREST? Use JWT manually is possible,
  // but easiest is to call auth.getUser().
  const supabaseUser = await supabase.auth.getUser(token).catch(()=>null);
  const user = supabaseUser?.data?.user;
  if (!user) return new Response('Unauthorized', {
    status: 401
  });
  // Ensure user is a room member
  const { data: membership, error: membershipErr } = await supabase.from('room_members').select('room_id').eq('room_id', room_id).eq('user_id', user.id).maybeSingle();
  if (membershipErr) {
    return new Response('Failed membership check', {
      status: 500
    });
  }
  if (!membership) {
    return new Response('Forbidden', {
      status: 403
    });
  }
  // Fetch last N messages
  const { data: messages, error: messagesErr } = await supabase.from('messages').select('sender_id, body, created_at').eq('room_id', room_id).order('created_at', {
    ascending: false
  }).limit(limitNum);
  if (messagesErr) {
    return new Response('Failed to load messages', {
      status: 500
    });
  }
  const ordered = (messages || []).slice().reverse();
  const transcript = ordered.map((m)=>{
    const who = m.sender_id === user.id ? 'You' : m.sender_id;
    return `[${new Date(m.created_at).toISOString()}] ${who}: ${m.body}`;
  }).join('\n');
  const openai = new OpenAI({
    apiKey: Deno.env.get('OPENAI_API_KEY')
  });
  if (!Deno.env.get('OPENAI_API_KEY')) {
    return new Response('OpenAI API key not configured', {
      status: 500
    });
  }
  const system = `You are an assistant that summarizes message threads for a messaging app.\n\nReturn a JSON object with keys: summary, key_points, action_items.\n- summary: 1-3 sentences\n- key_points: array of short bullets (strings)\n- action_items: array of action item strings (may be empty)\n\nBe concise and avoid inventing facts.`;
  const prompt = `THREAD:\n${transcript}`;
  const completion = await openai.chat.completions.create({
    model: 'gpt-4o-mini',
    messages: [
      {
        role: 'system',
        content: system
      },
      {
        role: 'user',
        content: prompt
      }
    ],
    response_format: {
      type: 'json_object'
    },
    temperature: 0.2
  });
  const content = completion.choices[0]?.message?.content;
  let parsed;
  try {
    parsed = content ? JSON.parse(content) : null;
  } catch  {
    // Fallback if model doesn't follow format
    parsed = {
      summary: content || '',
      key_points: [],
      action_items: []
    };
  }
  return new Response(JSON.stringify({
    room_id,
    ...parsed
  }), {
    headers: {
      'Content-Type': 'application/json'
    }
  });
});

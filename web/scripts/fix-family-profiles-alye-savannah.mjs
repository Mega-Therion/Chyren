import { neon } from '@neondatabase/serverless'

/**
 * Idempotent fix for family_profiles rows related to:
 * - Alye Lauren Muldoon (goes by Alye)
 * - Savannah (Alye's half-sister; Suszie's other daughter)
 * - Suzanne (Suszie) Muldoon (their mom)
 *
 * Usage:
 *   OMEGA_DB_URL='postgres://...' node scripts/fix-family-profiles-alye-savannah.mjs
 */

function requireEnv(name) {
  const v = process.env[name]
  if (!v || !String(v).trim()) throw new Error(`Missing required env var: ${name}`)
  return String(v).trim()
}

async function main() {
  const sql = neon(requireEnv('OMEGA_DB_URL'))

  // Discover rows we can safely rewrite (historically duplicated as "Lauren A. Muldoon").
  const candidates = await sql`
    SELECT id, name, last_name
    FROM family_profiles
    WHERE lower(last_name) LIKE '%muldoon%'
      AND (lower(name) LIKE 'lauren%' OR lower(name) LIKE 'alye%' OR lower(name) LIKE 'savannah%')
    ORDER BY id
  `

  const suszieRow = (
    await sql`
      SELECT id, name, last_name
      FROM family_profiles
      WHERE lower(last_name) LIKE '%muldoon%'
        AND (lower(name) LIKE 'suz%' OR lower(name) LIKE 'sus%')
      ORDER BY id
      LIMIT 1
    `
  )[0]

  const laurenLike = candidates.filter((r) =>
    String(r.name || '').toLowerCase().startsWith('lauren'),
  )

  // Prefer the smallest id as Alye, and next as Savannah to keep stable ids for context.
  const alyeId =
    laurenLike[0]?.id ??
    candidates.find((r) => String(r.name || '').toLowerCase().startsWith('alye'))?.id
  const savannahId =
    laurenLike[1]?.id ??
    candidates.find((r) => String(r.name || '').toLowerCase().startsWith('savannah'))?.id

  if (!alyeId || !savannahId) {
    throw new Error(
      `Could not find both Alye and Savannah candidate rows. Found: ${JSON.stringify(candidates)}`,
    )
  }

  await sql`BEGIN`
  try {
    // Alye
    await sql`
      UPDATE family_profiles
      SET name = ${'Alye'},
          last_name = ${'Lauren Muldoon'},
          relationship = ${'cousin (paternal side, through Suszie Muldoon)'},
          partner = ${'Justin "Jay" Schwartz'},
          children = ${JSON.stringify(["Deacon 'Bean' Schwartz (b. ~2022)"])},
          ry_notes = ${"Alye is RY's first cousin — Suzanne (Suszie) Muldoon's daughter. Her full name is Alye Lauren Muldoon and she goes by Alye. She is Jay's partner and Deacon 'Bean' Schwartz's mother (b. ~2022). She has a half-sister named Savannah (also Suszie's daughter). There is no sister named Lauren."},
          notes_for_omega = ${'Technically literate and comfortable with software; can engage with architecture if she wants. Follow her lead.'}
      WHERE id = ${alyeId}
    `

    // Suszie (Suzanne)
    if (!suszieRow?.id) {
      throw new Error('Could not find Suszie/Suzanne Muldoon row to update.')
    }
    await sql`
      UPDATE family_profiles
      SET relationship = ${"paternal aunt (Robo's sister, Alye and Savannah's mom)"},
          ry_notes = ${"Suzanne (Suszie) Muldoon is RY's paternal aunt — Robo's sister, and the mother of Alye Lauren Muldoon and Savannah. She's an LPC and now retired. Academic and empathetic but not uptight about it."},
          notes_for_omega = ${"Relationship to RY: paternal aunt (Robo's sister). She is Alye and Savannah's mom."}
      WHERE id = ${suszieRow.id}
    `

    // Savannah
    await sql`
      UPDATE family_profiles
      SET name = ${'Savannah'},
          last_name = ${'Muldoon'},
          relationship = ${'cousin (paternal side, through Suszie Muldoon)'},
          partner = ${null},
          children = ${null},
          ry_notes = ${"Savannah is RY's cousin — Suzanne (Suszie) Muldoon's daughter. She is Alye Lauren Muldoon's half-sister. There is no sister named Lauren."},
          notes_for_omega = ${'Cousin via Suszie Muldoon; half-sister of Alye Lauren Muldoon.'}
      WHERE id = ${savannahId}
    `

    await sql`COMMIT`
  } catch (e) {
    await sql`ROLLBACK`
    throw e
  }

  const after = await sql`
    SELECT id, name, last_name, relationship, partner, children
    FROM family_profiles
    WHERE id IN (${alyeId}, ${suszieRow.id}, ${savannahId})
    ORDER BY id
  `

  // Minimal confirmation output for operators.
  console.log(JSON.stringify(after, null, 2))
}

main().catch((err) => {
  console.error(err)
  process.exit(1)
})

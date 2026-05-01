//! Mathematics knowledge programs for the Neocortex.
//!
//! Covers: College Algebra 1 & 2, Trigonometry, Calculus, Nonlinear ODEs,
//! Vortex-Based Mathematics, Linear Algebra, Number Theory, Complex Analysis,
//! Topology, Information Theory, and Advanced topics.
//!
//! Each program is a self-contained JSON payload. Annotations and hints are
//! embedded alongside formulas for direct use by the reasoning pipeline.

use crate::{Domain, Program, ProgramLibrary};

pub fn register_all(lib: &mut ProgramLibrary) {
    lib.register(algebra_program());
    lib.register(trigonometry_program());
    lib.register(calculus_program());
    lib.register(differential_equations_program());
    lib.register(vortex_mathematics_program());
    lib.register(advanced_mathematics_program());
}

// ── College Algebra ───────────────────────────────────────────────────────────

pub fn algebra_program() -> Program {
    Program::from_knowledge(
        Domain::Custom("math_algebra".into()),
        "1.0.0",
        &serde_json::json!({
            "domain_overview": "College Algebra 1 & 2 — the complete symbolic toolkit from foundational laws through matrices, conics, and complex numbers.",

            "fundamental_laws": [
                {
                    "name": "Commutative Law",
                    "addition": "a + b = b + a",
                    "multiplication": "a · b = b · a",
                    "hint": "Order does not matter for + and ×. Does NOT apply to subtraction or division."
                },
                {
                    "name": "Associative Law",
                    "addition": "(a + b) + c = a + (b + c)",
                    "multiplication": "(a · b) · c = a · (b · c)",
                    "hint": "Grouping does not matter for + and ×."
                },
                {
                    "name": "Distributive Law",
                    "formula": "a(b + c) = ab + ac",
                    "hint": "The single most-used law in algebra. FOIL is just distributive applied twice: (a+b)(c+d) = ac + ad + bc + bd"
                },
                {
                    "name": "Identity Elements",
                    "additive": "a + 0 = a",
                    "multiplicative": "a · 1 = a"
                },
                {
                    "name": "Inverse Elements",
                    "additive": "a + (−a) = 0",
                    "multiplicative": "a · (1/a) = 1 for a ≠ 0"
                }
            ],

            "order_of_operations": {
                "acronym": "PEMDAS / BODMAS",
                "order": ["Parentheses / Brackets", "Exponents / Orders", "Multiplication & Division (left to right)", "Addition & Subtraction (left to right)"],
                "hint": "Multiplication and division have equal precedence — work LEFT to RIGHT. Same for addition and subtraction. 6 ÷ 2 × 3 = 9, not 1."
            },

            "exponent_rules": [
                { "rule": "Product rule", "formula": "a^m · a^n = a^(m+n)", "hint": "Same base → add exponents" },
                { "rule": "Quotient rule", "formula": "a^m / a^n = a^(m−n)", "hint": "Same base → subtract exponents" },
                { "rule": "Power rule", "formula": "(a^m)^n = a^(mn)", "hint": "Power of a power → multiply exponents" },
                { "rule": "Zero exponent", "formula": "a^0 = 1 (a ≠ 0)", "hint": "Anything to the zero is 1. 0^0 is undefined/indeterminate." },
                { "rule": "Negative exponent", "formula": "a^(−n) = 1/a^n", "hint": "Negative exponent = reciprocal. Move to denominator, flip sign." },
                { "rule": "Fractional exponent", "formula": "a^(m/n) = (ⁿ√a)^m = ⁿ√(a^m)", "hint": "Denominator = root, numerator = power. a^(1/2) = √a." },
                { "rule": "Product to power", "formula": "(ab)^n = a^n · b^n" },
                { "rule": "Quotient to power", "formula": "(a/b)^n = a^n / b^n" }
            ],

            "radical_rules": [
                { "rule": "Product", "formula": "√(ab) = √a · √b" },
                { "rule": "Quotient", "formula": "√(a/b) = √a / √b" },
                { "rule": "Simplify", "formula": "√(a²) = |a|", "hint": "Always take absolute value when simplifying even roots of variables." },
                { "rule": "Rationalize denominator", "formula": "1/√a = √a/a", "hint": "Multiply top and bottom by the radical. For conjugates: multiply by (a − b√c)/(a − b√c)." }
            ],

            "factoring": [
                { "type": "Greatest Common Factor", "formula": "ab + ac = a(b + c)", "hint": "Always try GCF first before any other method." },
                { "type": "Difference of Squares", "formula": "a² − b² = (a+b)(a−b)", "hint": "Two perfect squares separated by a minus. Sum of squares a²+b² does NOT factor over ℝ." },
                { "type": "Perfect Square Trinomial", "formula": "a² ± 2ab + b² = (a ± b)²" },
                { "type": "Sum of Cubes", "formula": "a³ + b³ = (a+b)(a² − ab + b²)", "hint": "SOAP: Same sign, Opposite sign, Always Positive" },
                { "type": "Difference of Cubes", "formula": "a³ − b³ = (a−b)(a² + ab + b²)" },
                { "type": "Trinomial (leading coeff = 1)", "formula": "x² + bx + c = (x+p)(x+q) where p+q=b, pq=c", "hint": "Find two numbers that multiply to c and add to b." },
                { "type": "Trinomial (leading coeff ≠ 1)", "formula": "ax² + bx + c: use AC method or grouping", "hint": "Multiply a·c. Find two numbers that multiply to ac and add to b. Split middle term." },
                { "type": "Grouping", "formula": "4 terms: factor pairs, then factor again", "hint": "Group first two and last two terms. Factor each group. If binomial factors match, factor that out." }
            ],

            "quadratic_equations": {
                "standard_form": "ax² + bx + c = 0",
                "quadratic_formula": "x = (−b ± √(b²−4ac)) / 2a",
                "discriminant": {
                    "formula": "Δ = b² − 4ac",
                    "cases": [
                        "Δ > 0 → two distinct real roots",
                        "Δ = 0 → one repeated real root (perfect square)",
                        "Δ < 0 → two complex conjugate roots (no real solutions)"
                    ],
                    "hint": "Check discriminant BEFORE solving. Tells you what kind of answer to expect."
                },
                "completing_the_square": {
                    "steps": [
                        "1. Move constant to right side: ax² + bx = −c",
                        "2. Divide by a: x² + (b/a)x = −c/a",
                        "3. Add (b/2a)² to both sides",
                        "4. Left side is (x + b/2a)² — take square roots"
                    ],
                    "hint": "Completing the square derives the quadratic formula and converts to vertex form."
                },
                "vertex_form": "y = a(x−h)² + k, vertex at (h, k)",
                "hint": "Vertex form immediately gives max/min value = k at x = h. If a > 0 opens up (min), a < 0 opens down (max)."
            },

            "functions": {
                "definition": "A function f: A → B assigns each element of A exactly one element of B.",
                "vertical_line_test": "A curve is a function iff every vertical line intersects it at most once.",
                "domain": "All valid inputs (x values). Watch for: division by zero, even roots of negatives, log of non-positive.",
                "range": "All possible outputs (y values).",
                "composition": "(f ∘ g)(x) = f(g(x)). Note: f ∘ g ≠ g ∘ f in general.",
                "inverse": {
                    "definition": "f⁻¹ exists iff f is one-to-one (passes horizontal line test).",
                    "find_inverse": "1. Replace f(x) with y. 2. Swap x and y. 3. Solve for y. 4. Replace y with f⁻¹(x).",
                    "key_property": "f(f⁻¹(x)) = x and f⁻¹(f(x)) = x",
                    "hint": "The graph of f⁻¹ is the reflection of f across the line y = x."
                },
                "transformations": [
                    "f(x) + k → shift up k",
                    "f(x) − k → shift down k",
                    "f(x − h) → shift right h",
                    "f(x + h) → shift left h",
                    "−f(x) → reflect over x-axis",
                    "f(−x) → reflect over y-axis",
                    "af(x), a > 1 → vertical stretch; 0 < a < 1 → vertical compression",
                    "f(bx), b > 1 → horizontal compression; 0 < b < 1 → horizontal stretch"
                ]
            },

            "polynomial_theorems": [
                { "name": "Remainder Theorem", "statement": "When p(x) is divided by (x−c), the remainder is p(c).", "hint": "Faster than long division when you just need the remainder." },
                { "name": "Factor Theorem", "statement": "(x−c) is a factor of p(x) iff p(c) = 0.", "hint": "Zero of the polynomial = factor of the polynomial." },
                { "name": "Rational Root Theorem", "statement": "If p/q (in lowest terms) is a rational root of aₙxⁿ + ... + a₀, then p | a₀ and q | aₙ.", "hint": "Lists ALL POSSIBLE rational roots. Test with synthetic division." },
                { "name": "Fundamental Theorem of Algebra", "statement": "Every polynomial of degree n ≥ 1 with complex coefficients has exactly n complex roots (counting multiplicity)." },
                { "name": "Conjugate Root Theorem", "statement": "If a+bi is a root of a polynomial with real coefficients, so is a−bi.", "hint": "Complex roots of real polynomials always come in conjugate pairs." }
            ],

            "systems_of_equations": {
                "methods": ["Substitution", "Elimination (addition)", "Matrix methods (Gaussian elimination, Cramer's rule)"],
                "cramers_rule": {
                    "2x2": "For ax+by=e, cx+dy=f: D=ad−bc, x=|e,b;f,d|/D, y=|a,e;c,f|/D",
                    "hint": "Use Cramer's rule only when D ≠ 0. If D = 0, system is either inconsistent or dependent."
                },
                "hint": "For 3 equations: elimination usually faster than substitution. Check solution in ALL original equations."
            },

            "logarithms_and_exponentials": {
                "definition": "logₐ(x) = y ↔ aʸ = x",
                "natural_log": "ln(x) = logₑ(x), e ≈ 2.71828",
                "common_log": "log(x) = log₁₀(x)",
                "log_rules": [
                    "logₐ(MN) = logₐM + logₐN (product rule)",
                    "logₐ(M/N) = logₐM − logₐN (quotient rule)",
                    "logₐ(Mⁿ) = n·logₐM (power rule)",
                    "logₐ(1) = 0",
                    "logₐ(a) = 1",
                    "logₐ(aˣ) = x",
                    "aˡᵒᵍₐˣ = x"
                ],
                "change_of_base": "logₐ(x) = ln(x)/ln(a) = log(x)/log(a)",
                "exponential_growth_decay": "A(t) = A₀·eᵏᵗ where k > 0 growth, k < 0 decay",
                "hint": "To solve exponential equations: take log of both sides. To solve log equations: exponentiate both sides. Always check domain (log undefined at x ≤ 0)."
            },

            "complex_numbers": {
                "definition": "z = a + bi where i = √(−1), i² = −1",
                "arithmetic": {
                    "addition": "(a+bi) + (c+di) = (a+c) + (b+d)i",
                    "multiplication": "(a+bi)(c+di) = (ac−bd) + (ad+bc)i",
                    "division": "(a+bi)/(c+di) = multiply top and bottom by conjugate (c−di)"
                },
                "modulus": "|z| = √(a²+b²)",
                "conjugate": "z̄ = a−bi, z·z̄ = |z|²",
                "polar_form": "z = r(cos θ + i sin θ) = re^(iθ), r = |z|, θ = arg(z)",
                "de_moivre": "zⁿ = rⁿ(cos(nθ) + i sin(nθ))",
                "eulers_formula": "e^(iθ) = cos θ + i sin θ",
                "hint": "Euler's formula is the most beautiful equation in mathematics: e^(iπ) + 1 = 0 connects e, i, π, 1, and 0."
            },

            "sequences_and_series": {
                "arithmetic": {
                    "nth_term": "aₙ = a₁ + (n−1)d",
                    "sum": "Sₙ = n/2 · (a₁ + aₙ) = n/2 · (2a₁ + (n−1)d)",
                    "hint": "d = common difference. Sum = average of first and last times number of terms."
                },
                "geometric": {
                    "nth_term": "aₙ = a₁ · rⁿ⁻¹",
                    "sum_finite": "Sₙ = a₁(1−rⁿ)/(1−r) for r ≠ 1",
                    "sum_infinite": "S∞ = a₁/(1−r) for |r| < 1",
                    "hint": "Infinite geometric series only converges when |r| < 1. The sum formula is one of the most useful in all of mathematics."
                },
                "binomial_theorem": {
                    "formula": "(a+b)ⁿ = Σₖ₌₀ⁿ C(n,k) aⁿ⁻ᵏ bᵏ",
                    "binomial_coefficient": "C(n,k) = n! / (k!(n−k)!) = 'n choose k'",
                    "pascal_triangle": "Each entry = sum of two entries above it.",
                    "hint": "Remember: exponents of a decrease, b increase, always sum to n. Signs alternate only if (a−b)ⁿ."
                }
            },

            "matrices": {
                "operations": {
                    "addition": "Add corresponding entries. Must have same dimensions.",
                    "scalar_multiplication": "Multiply every entry by scalar.",
                    "matrix_multiplication": "C[i,j] = Σₖ A[i,k]·B[k,j]. # columns of A must equal # rows of B.",
                    "hint": "Matrix multiplication is NOT commutative: AB ≠ BA in general."
                },
                "determinant_2x2": "det[a,b;c,d] = ad − bc",
                "determinant_3x3": "Cofactor expansion along any row or column.",
                "inverse_2x2": "A⁻¹ = (1/det(A)) · [d,−b;−c,a]",
                "properties": [
                    "det(AB) = det(A)·det(B)",
                    "det(Aᵀ) = det(A)",
                    "A⁻¹ exists iff det(A) ≠ 0",
                    "(AB)⁻¹ = B⁻¹A⁻¹",
                    "(Aᵀ)⁻¹ = (A⁻¹)ᵀ"
                ],
                "hint": "The determinant encodes signed area (2D) or signed volume (3D) of the transformation."
            },

            "conic_sections": {
                "circle": { "standard": "(x−h)² + (y−k)² = r²", "center": "(h,k)", "radius": "r" },
                "parabola": {
                    "vertical": "y = a(x−h)² + k, vertex (h,k), opens up if a>0",
                    "horizontal": "x = a(y−k)² + h, vertex (h,k), opens right if a>0",
                    "hint": "Focus-directrix definition: set of points equidistant from focus and directrix."
                },
                "ellipse": {
                    "standard": "(x−h)²/a² + (y−k)²/b² = 1",
                    "key": "c² = a²−b² (a>b), foci at distance c from center along major axis"
                },
                "hyperbola": {
                    "horizontal": "(x−h)²/a² − (y−k)²/b² = 1",
                    "vertical": "(y−k)²/a² − (x−h)²/b² = 1",
                    "key": "c² = a²+b², asymptotes y−k = ±(b/a)(x−h)"
                },
                "hint": "All conics are cross-sections of a double cone. The eccentricity e classifies them: e=0 circle, e<1 ellipse, e=1 parabola, e>1 hyperbola."
            }
        }),
        "College Algebra 1 & 2: laws, exponents, factoring, quadratics, functions, polynomials, logarithms, complex numbers, matrices, conics",
        0.92,
    )
    .expect("math_algebra program")
}

// ── Trigonometry ──────────────────────────────────────────────────────────────

pub fn trigonometry_program() -> Program {
    Program::from_knowledge(
        Domain::Custom("math_trigonometry".into()),
        "1.0.0",
        &serde_json::json!({
            "domain_overview": "Trigonometry — the mathematics of angles, triangles, and periodic phenomena. Foundation for calculus, physics, and signal processing.",

            "unit_circle": {
                "definition": "Circle of radius 1 centered at origin. Point (x,y) at angle θ gives cos θ = x, sin θ = y.",
                "key_angles_degrees_radians": [
                    { "deg": 0, "rad": "0", "sin": "0", "cos": "1", "tan": "0" },
                    { "deg": 30, "rad": "π/6", "sin": "1/2", "cos": "√3/2", "tan": "1/√3 = √3/3" },
                    { "deg": 45, "rad": "π/4", "sin": "√2/2", "cos": "√2/2", "tan": "1" },
                    { "deg": 60, "rad": "π/3", "sin": "√3/2", "cos": "1/2", "tan": "√3" },
                    { "deg": 90, "rad": "π/2", "sin": "1", "cos": "0", "tan": "undefined" },
                    { "deg": 180, "rad": "π", "sin": "0", "cos": "-1", "tan": "0" },
                    { "deg": 270, "rad": "3π/2", "sin": "-1", "cos": "0", "tan": "undefined" },
                    { "deg": 360, "rad": "2π", "sin": "0", "cos": "1", "tan": "0" }
                ],
                "memory_trick": "All Students Take Calculus — quadrant sign rule: Q1 all positive, Q2 sin positive, Q3 tan positive, Q4 cos positive.",
                "conversion": "degrees × π/180 = radians. radians × 180/π = degrees."
            },

            "trig_functions": {
                "primary": {
                    "sin": "opposite/hypotenuse",
                    "cos": "adjacent/hypotenuse",
                    "tan": "opposite/adjacent = sin/cos"
                },
                "reciprocal": {
                    "csc": "1/sin = hypotenuse/opposite",
                    "sec": "1/cos = hypotenuse/adjacent",
                    "cot": "1/tan = cos/sin = adjacent/opposite"
                },
                "sohcahtoa": "SOH: sin=Opp/Hyp, CAH: cos=Adj/Hyp, TOA: tan=Opp/Adj",
                "hint": "Memorize SOH-CAH-TOA. Everything else derives from it."
            },

            "fundamental_identities": {
                "pythagorean": [
                    "sin²θ + cos²θ = 1  ← most important identity in trig",
                    "tan²θ + 1 = sec²θ  ← divide Pythagorean by cos²θ",
                    "1 + cot²θ = csc²θ  ← divide Pythagorean by sin²θ"
                ],
                "reciprocal": ["csc θ = 1/sin θ", "sec θ = 1/cos θ", "cot θ = 1/tan θ"],
                "quotient": ["tan θ = sin θ/cos θ", "cot θ = cos θ/sin θ"],
                "even_odd": [
                    "sin(−θ) = −sin θ (odd)",
                    "cos(−θ) = cos θ (even)",
                    "tan(−θ) = −tan θ (odd)"
                ],
                "hint": "sin²θ + cos²θ = 1 is the Pythagorean theorem on the unit circle. Never forget it."
            },

            "sum_and_difference": {
                "formulas": [
                    "sin(A±B) = sin A cos B ± cos A sin B",
                    "cos(A±B) = cos A cos B ∓ sin A sin B  ← note the ∓",
                    "tan(A±B) = (tan A ± tan B)/(1 ∓ tan A tan B)"
                ],
                "hint": "For cos: same operation with minus, different operation with plus (∓). Opposite of sin."
            },

            "double_angle": {
                "formulas": [
                    "sin(2θ) = 2 sin θ cos θ",
                    "cos(2θ) = cos²θ − sin²θ = 2cos²θ − 1 = 1 − 2sin²θ  ← three forms",
                    "tan(2θ) = 2tan θ / (1 − tan²θ)"
                ],
                "hint": "Three forms of cos(2θ) — choose whichever matches what you have in the problem."
            },

            "half_angle": {
                "formulas": [
                    "sin(θ/2) = ±√((1−cosθ)/2)",
                    "cos(θ/2) = ±√((1+cosθ)/2)",
                    "tan(θ/2) = sin θ/(1+cos θ) = (1−cos θ)/sin θ"
                ],
                "hint": "Sign of sin(θ/2) and cos(θ/2) depends on the quadrant of θ/2."
            },

            "product_to_sum": {
                "formulas": [
                    "sin A cos B = (1/2)[sin(A+B) + sin(A−B)]",
                    "cos A cos B = (1/2)[cos(A−B) + cos(A+B)]",
                    "sin A sin B = (1/2)[cos(A−B) − cos(A+B)]"
                ],
                "hint": "Used in Fourier analysis and integration of products of trig functions."
            },

            "sum_to_product": [
                "sin A + sin B = 2 sin((A+B)/2) cos((A−B)/2)",
                "sin A − sin B = 2 cos((A+B)/2) sin((A−B)/2)",
                "cos A + cos B = 2 cos((A+B)/2) cos((A−B)/2)",
                "cos A − cos B = −2 sin((A+B)/2) sin((A−B)/2)"
            ],

            "inverse_trig_functions": {
                "domains_and_ranges": [
                    { "function": "arcsin", "domain": "[−1,1]", "range": "[−π/2, π/2]" },
                    { "function": "arccos", "domain": "[−1,1]", "range": "[0, π]" },
                    { "function": "arctan", "domain": "(−∞,∞)", "range": "(−π/2, π/2)" }
                ],
                "hint": "arcsin and arctan return values in [−π/2, π/2] (quadrants I and IV). arccos returns [0,π] (quadrants I and II).",
                "key_values": "arcsin(1/2)=π/6, arcsin(√2/2)=π/4, arcsin(√3/2)=π/3, arctan(1)=π/4, arctan(√3)=π/3"
            },

            "law_of_sines": {
                "formula": "a/sin A = b/sin B = c/sin C = 2R",
                "use_when": "AAS, ASA, SSA (ambiguous case)",
                "ambiguous_case_hint": "SSA may give 0, 1, or 2 triangles. Check: if a < b·sin A → no triangle. If a = b·sin A → right triangle. If a ≥ b → one triangle. If b·sin A < a < b → two triangles."
            },

            "law_of_cosines": {
                "formula": "a² = b² + c² − 2bc cos A  (and cyclically)",
                "solve_for_angle": "cos A = (b²+c²−a²)/(2bc)",
                "use_when": "SAS or SSS",
                "hint": "Generalization of Pythagorean theorem. When C=90°, cos C=0 and you recover a²+b²=c²."
            },

            "polar_coordinates": {
                "conversion": {
                    "to_rectangular": "x = r cos θ, y = r sin θ",
                    "to_polar": "r = √(x²+y²), θ = arctan(y/x) (adjust quadrant)"
                },
                "common_curves": [
                    "r = a → circle radius a",
                    "r = 2a cos θ → circle through origin",
                    "r = a(1 + cos θ) → cardioid",
                    "r = a cos(nθ) → rose with n petals (n odd) or 2n petals (n even)"
                ]
            },

            "fourier_basics": {
                "concept": "Any periodic function can be written as a sum of sines and cosines.",
                "formula": "f(x) = a₀/2 + Σₙ[aₙ cos(nπx/L) + bₙ sin(nπx/L)]",
                "coefficients": "aₙ = (1/L)∫₋ₗᴸ f(x)cos(nπx/L)dx, bₙ = (1/L)∫₋ₗᴸ f(x)sin(nπx/L)dx",
                "hint": "Fourier series is why music can be decomposed into frequencies. Every sound is a sum of pure tones."
            }
        }),
        "Trigonometry: unit circle, identities, sum/difference/double/half angle formulas, inverse trig, laws of sines and cosines, polar, Fourier basics",
        0.92,
    )
    .expect("math_trigonometry program")
}

// ── Calculus ──────────────────────────────────────────────────────────────────

pub fn calculus_program() -> Program {
    Program::from_knowledge(
        Domain::Custom("math_calculus".into()),
        "1.0.0",
        &serde_json::json!({
            "domain_overview": "Calculus — the mathematics of change and accumulation. Differential calculus: instantaneous rates. Integral calculus: areas and totals. Together via the Fundamental Theorem.",

            "limits": {
                "informal": "lim_{x→a} f(x) = L means f(x) gets arbitrarily close to L as x approaches a (but x ≠ a).",
                "formal_epsilon_delta": "∀ε>0 ∃δ>0: 0<|x−a|<δ ⟹ |f(x)−L|<ε",
                "properties": [
                    "lim(f ± g) = lim f ± lim g",
                    "lim(f · g) = lim f · lim g",
                    "lim(f/g) = lim f / lim g  (if lim g ≠ 0)",
                    "lim(f(g(x))) = f(lim g(x))  (if f is continuous)"
                ],
                "special_limits": [
                    "lim_{x→0} sin(x)/x = 1",
                    "lim_{x→0} (1−cos x)/x = 0",
                    "lim_{x→∞} (1 + 1/x)^x = e",
                    "lim_{x→0} (1 + x)^(1/x) = e"
                ],
                "lhopital_rule": {
                    "statement": "If lim f/g gives 0/0 or ∞/∞, then lim f/g = lim f'/g' (if that limit exists).",
                    "hint": "Only apply when you get an indeterminate form (0/0, ∞/∞, 0·∞, ∞−∞, 0⁰, 1^∞, ∞⁰). Convert other forms first."
                },
                "squeeze_theorem": "If g(x) ≤ f(x) ≤ h(x) and lim g = lim h = L, then lim f = L.",
                "hint": "Limits describe behavior near a point, not AT the point. A function can fail to exist at a point but still have a limit there."
            },

            "derivatives": {
                "definition": "f'(x) = lim_{h→0} [f(x+h) − f(x)] / h = instantaneous rate of change",
                "geometric_meaning": "Slope of the tangent line to f at x.",
                "basic_rules": [
                    { "rule": "Constant", "formula": "d/dx[c] = 0" },
                    { "rule": "Power", "formula": "d/dx[xⁿ] = nxⁿ⁻¹", "hint": "Bring down the power, reduce exponent by 1. Works for any real n." },
                    { "rule": "Constant multiple", "formula": "d/dx[cf] = cf'" },
                    { "rule": "Sum/difference", "formula": "d/dx[f ± g] = f' ± g'" },
                    { "rule": "Product rule", "formula": "d/dx[fg] = f'g + fg'", "hint": "First times derivative of second PLUS second times derivative of first." },
                    { "rule": "Quotient rule", "formula": "d/dx[f/g] = (f'g − fg')/g²", "hint": "Low d-high minus high d-low over low squared. (LdH − HdL)/L²" },
                    { "rule": "Chain rule", "formula": "d/dx[f(g(x))] = f'(g(x)) · g'(x)", "hint": "Derivative of outside (leave inside alone) times derivative of inside. Most important rule." }
                ],
                "trig_derivatives": [
                    "d/dx[sin x] = cos x",
                    "d/dx[cos x] = −sin x",
                    "d/dx[tan x] = sec²x",
                    "d/dx[cot x] = −csc²x",
                    "d/dx[sec x] = sec x tan x",
                    "d/dx[csc x] = −csc x cot x"
                ],
                "exponential_log_derivatives": [
                    "d/dx[eˣ] = eˣ  ← only function equal to its own derivative",
                    "d/dx[aˣ] = aˣ ln a",
                    "d/dx[ln x] = 1/x",
                    "d/dx[logₐ x] = 1/(x ln a)"
                ],
                "inverse_trig_derivatives": [
                    "d/dx[arcsin x] = 1/√(1−x²)",
                    "d/dx[arccos x] = −1/√(1−x²)",
                    "d/dx[arctan x] = 1/(1+x²)"
                ],
                "implicit_differentiation": {
                    "method": "Differentiate both sides with respect to x. When differentiating y terms, multiply by dy/dx. Solve for dy/dx.",
                    "hint": "Use when equation defines y implicitly. e.g., x² + y² = 25 → 2x + 2y(dy/dx) = 0 → dy/dx = −x/y"
                },
                "higher_derivatives": "f''(x) = d²f/dx² = second derivative = rate of change of rate of change = concavity.",
                "hint": "f' > 0 → increasing, f' < 0 → decreasing, f' = 0 → critical point. f'' > 0 → concave up, f'' < 0 → concave down."
            },

            "derivative_applications": {
                "critical_points": "f'(c) = 0 or f'(c) undefined. Candidates for local max/min.",
                "first_derivative_test": "f' changes + to − at c → local max. f' changes − to + → local min.",
                "second_derivative_test": "f'(c)=0 and f''(c)<0 → local max. f''(c)>0 → local min. f''(c)=0 → inconclusive.",
                "mean_value_theorem": {
                    "statement": "If f is continuous on [a,b] and differentiable on (a,b), then ∃c ∈ (a,b): f'(c) = (f(b)−f(a))/(b−a)",
                    "geometric_meaning": "There is a point where the tangent slope equals the secant slope.",
                    "hint": "MVT is the rigorous foundation of most calculus theorems. Rolle's Theorem is the special case f(a)=f(b)."
                },
                "related_rates": {
                    "method": "1. Draw and label. 2. Write equation relating quantities. 3. Differentiate implicitly with respect to t. 4. Plug in known values.",
                    "hint": "Differentiate BEFORE plugging in values. Once you substitute a quantity that's changing, you lose the derivative."
                },
                "optimization": {
                    "method": "1. Express quantity to optimize as function of one variable. 2. Find domain. 3. Find critical points. 4. Check endpoints and critical points.",
                    "hint": "Real optimization problems always have physical constraints — use them to reduce to one variable."
                },
                "linearization": "L(x) = f(a) + f'(a)(x−a). Tangent line approximation near x=a.",
                "newtons_method": "xₙ₊₁ = xₙ − f(xₙ)/f'(xₙ). Iterative root-finding algorithm."
            },

            "integration": {
                "definition": "∫f(x)dx = F(x) + C where F'(x) = f(x). Antiderivative.",
                "riemann_sum": "∫ₐᵇ f(x)dx = lim_{n→∞} Σᵢ f(xᵢ*)Δx. Signed area under curve.",
                "fundamental_theorem": {
                    "part_1": "If F(x) = ∫ₐˣ f(t)dt, then F'(x) = f(x). Differentiation and integration are inverses.",
                    "part_2": "∫ₐᵇ f(x)dx = F(b) − F(a) where F'=f. Evaluation theorem.",
                    "hint": "FTC Part 2 is how you actually compute definite integrals. It connects the antiderivative to area."
                },
                "basic_integrals": [
                    "∫xⁿdx = xⁿ⁺¹/(n+1) + C (n ≠ −1)",
                    "∫x⁻¹dx = ∫(1/x)dx = ln|x| + C",
                    "∫eˣdx = eˣ + C",
                    "∫aˣdx = aˣ/ln(a) + C",
                    "∫sin x dx = −cos x + C",
                    "∫cos x dx = sin x + C",
                    "∫sec²x dx = tan x + C",
                    "∫csc²x dx = −cot x + C",
                    "∫sec x tan x dx = sec x + C",
                    "∫1/√(1−x²)dx = arcsin x + C",
                    "∫1/(1+x²)dx = arctan x + C"
                ],
                "u_substitution": {
                    "method": "Let u = g(x), du = g'(x)dx. Transforms ∫f(g(x))g'(x)dx into ∫f(u)du.",
                    "hint": "Look for a function and its derivative inside the integral. The derivative doesn't have to match exactly — off by a constant is fine."
                },
                "integration_by_parts": {
                    "formula": "∫u dv = uv − ∫v du",
                    "LIATE": "Choose u in order: Logarithms, Inverse trig, Algebraic (polynomial), Trigonometric, Exponential. What's earlier in LIATE = u.",
                    "hint": "Tabular method for repeated integration by parts: set up table of derivatives of u and integrals of dv, alternate signs."
                },
                "trig_substitution": {
                    "√(a²−x²)": "Let x = a sin θ",
                    "√(a²+x²)": "Let x = a tan θ",
                    "√(x²−a²)": "Let x = a sec θ",
                    "hint": "Trig sub converts radical expressions into pure trig. Draw the right triangle to convert back."
                },
                "partial_fractions": {
                    "use_when": "Rational function (polynomial/polynomial) where denominator degree > numerator.",
                    "method": "Factor denominator. Decompose into sum of simpler fractions with unknown constants. Solve for constants.",
                    "forms": [
                        "Linear factor (ax+b): A/(ax+b)",
                        "Repeated linear (ax+b)ⁿ: A/(ax+b) + B/(ax+b)² + ...",
                        "Irreducible quadratic (ax²+bx+c): (Ax+B)/(ax²+bx+c)"
                    ]
                }
            },

            "applications_of_integration": {
                "area_between_curves": "∫ₐᵇ [f(x)−g(x)]dx where f ≥ g. Split at intersection points.",
                "volume_disk_washer": "V = π∫ₐᵇ [R(x)²−r(x)²]dx (washer method, rotation about x-axis)",
                "volume_shell": "V = 2π∫ₐᵇ x·f(x)dx (shell method, rotation about y-axis)",
                "arc_length": "L = ∫ₐᵇ √(1 + [f'(x)]²)dx",
                "surface_area": "S = 2π∫ₐᵇ f(x)√(1 + [f'(x)]²)dx",
                "work": "W = ∫ₐᵇ F(x)dx",
                "average_value": "f_avg = (1/(b−a))∫ₐᵇ f(x)dx"
            },

            "sequences_and_series_calculus": {
                "convergence_tests": [
                    { "test": "Divergence Test", "condition": "If lim aₙ ≠ 0, series diverges. If lim aₙ = 0, inconclusive." },
                    { "test": "Integral Test", "condition": "Σaₙ converges iff ∫f(x)dx converges (f continuous, positive, decreasing)" },
                    { "test": "p-Series", "formula": "Σ 1/nᵖ converges iff p > 1" },
                    { "test": "Comparison Test", "condition": "Compare with known convergent/divergent series" },
                    { "test": "Limit Comparison", "condition": "If lim aₙ/bₙ = c > 0 (finite), both converge or both diverge" },
                    { "test": "Alternating Series Test", "condition": "Σ(−1)ⁿbₙ converges if bₙ decreasing → 0" },
                    { "test": "Ratio Test", "formula": "L = lim|aₙ₊₁/aₙ|. L<1 converges, L>1 diverges, L=1 inconclusive. Best for factorials and exponentials." },
                    { "test": "Root Test", "formula": "L = lim|aₙ|^(1/n). Same interpretation as ratio test." }
                ],
                "power_series": {
                    "form": "Σcₙ(x−a)ⁿ",
                    "radius_of_convergence": "R = 1/lim|cₙ₊₁/cₙ| or 1/lim|cₙ|^(1/n)",
                    "hint": "A power series converges absolutely for |x−a| < R, diverges for |x−a| > R. Check endpoints separately."
                },
                "taylor_maclaurin": {
                    "taylor": "f(x) = Σₙ f⁽ⁿ⁾(a)/n! · (x−a)ⁿ",
                    "maclaurin": "Σₙ f⁽ⁿ⁾(0)/n! · xⁿ (a=0 case)",
                    "common_series": [
                        "eˣ = Σ xⁿ/n! = 1 + x + x²/2! + x³/3! + ...",
                        "sin x = Σ (−1)ⁿx^(2n+1)/(2n+1)! = x − x³/6 + x⁵/120 − ...",
                        "cos x = Σ (−1)ⁿx^(2n)/(2n)! = 1 − x²/2 + x⁴/24 − ...",
                        "1/(1−x) = Σ xⁿ = 1 + x + x² + ... for |x|<1",
                        "ln(1+x) = Σ (−1)ⁿ⁺¹xⁿ/n = x − x²/2 + x³/3 − ..."
                    ],
                    "hint": "Memorize eˣ, sin x, cos x, and 1/(1−x) series. Others can be derived by substitution, differentiation, or integration."
                }
            },

            "multivariable_calculus": {
                "partial_derivatives": "∂f/∂x: differentiate with respect to x, treat all other variables as constants.",
                "gradient": "∇f = (∂f/∂x, ∂f/∂y, ∂f/∂z). Points in direction of steepest ascent. |∇f| is the rate of steepest ascent.",
                "directional_derivative": "Dᵤf = ∇f · û (dot product with unit vector û).",
                "critical_points_2D": "∇f = 0. Second derivative test: D = fₓₓfᵧᵧ − fₓᵧ². D>0,fₓₓ>0→min; D>0,fₓₓ<0→max; D<0→saddle.",
                "lagrange_multipliers": "Maximize f subject to g=0: ∇f = λ∇g. The λ is the Lagrange multiplier. System: ∇f = λ∇g, g(x,y)=0.",
                "hint": "Lagrange multipliers: the gradient of the objective must be parallel to the gradient of the constraint at the optimum."
            },

            "vector_calculus_theorems": {
                "greens_theorem": "∮_C (P dx + Q dy) = ∬_D (∂Q/∂x − ∂P/∂y) dA. Converts line integral to area integral.",
                "stokes_theorem": "∮_C F·dr = ∬_S (∇×F)·dS. Generalizes Green's theorem to surfaces.",
                "divergence_theorem": "∯_S F·dS = ∭_V (∇·F) dV. Flux through closed surface = integral of divergence inside.",
                "hint": "These three theorems are all special cases of the generalized Stokes theorem: ∫_∂Ω ω = ∫_Ω dω."
            }
        }),
        "Calculus: limits, L'Hôpital, derivatives, chain rule, integrals, FTC, integration techniques, series convergence, Taylor series, multivariable, vector calculus theorems",
        0.95,
    )
    .expect("math_calculus program")
}

// ── Differential Equations ────────────────────────────────────────────────────

pub fn differential_equations_program() -> Program {
    Program::from_knowledge(
        Domain::Custom("math_differential_equations".into()),
        "1.0.0",
        &serde_json::json!({
            "domain_overview": "Differential equations — equations relating functions to their derivatives. The native language of physics, engineering, biology, and the Lindblad master equation.",

            "classification": {
                "order": "Order = highest derivative present. First order: y', second order: y'', etc.",
                "linearity": "Linear: no products of y and its derivatives, no nonlinear functions of y. Nonlinear: everything else.",
                "autonomous": "Autonomous: right-hand side does not explicitly depend on t. dy/dt = f(y). Phase line analysis applies.",
                "hint": "Always classify before solving. The classification determines which method works."
            },

            "first_order_ODEs": {
                "separable": {
                    "form": "dy/dx = f(x)g(y)",
                    "method": "Separate: dy/g(y) = f(x)dx. Integrate both sides.",
                    "hint": "Separate COMPLETELY before integrating. Don't forget the constant of integration."
                },
                "linear_first_order": {
                    "form": "dy/dx + P(x)y = Q(x)",
                    "integrating_factor": "μ(x) = e^(∫P(x)dx)",
                    "solution": "y = (1/μ) ∫μQ dx",
                    "hint": "Multiply both sides by μ. Left side becomes d/dx[μy]. Then integrate."
                },
                "exact_equations": {
                    "form": "M(x,y)dx + N(x,y)dy = 0",
                    "exactness_condition": "∂M/∂y = ∂N/∂x",
                    "method": "Find F(x,y) where ∂F/∂x = M and ∂F/∂y = N. Solution: F(x,y) = C.",
                    "if_not_exact": "Find integrating factor μ(x) or μ(y) to make it exact."
                },
                "bernoulli_equation": {
                    "form": "dy/dx + P(x)y = Q(x)yⁿ",
                    "substitution": "v = y^(1−n), transforms to linear equation in v",
                    "hint": "n=0 is already linear. n=1 is separable."
                }
            },

            "second_order_linear_ODEs": {
                "homogeneous_constant_coefficients": {
                    "form": "ay'' + by' + cy = 0",
                    "characteristic_equation": "ar² + br + c = 0",
                    "cases": [
                        "Two distinct real roots r₁,r₂: y = C₁e^(r₁x) + C₂e^(r₂x)",
                        "Repeated real root r: y = (C₁ + C₂x)e^(rx)",
                        "Complex roots r = α±βi: y = e^(αx)(C₁cos(βx) + C₂sin(βx))"
                    ],
                    "hint": "The characteristic equation is the key. Solve it first, then write the general solution based on root type."
                },
                "nonhomogeneous": {
                    "general_solution": "y = yₕ + yₚ (homogeneous + particular solution)",
                    "undetermined_coefficients": {
                        "use_when": "Right-hand side is polynomial, exponential, sin/cos, or products/sums of these.",
                        "guess_forms": [
                            "If rhs = Pₙ(x): guess yₚ = xˢQₙ(x) (s=0 unless r=0 is root of characteristic eq)",
                            "If rhs = Pₙ(x)eᵃˣ: guess yₚ = xˢQₙ(x)eᵃˣ",
                            "If rhs = eᵃˣcos(βx) or eᵃˣsin(βx): guess yₚ = xˢeᵃˣ(A cos(βx) + B sin(βx))"
                        ],
                        "hint": "The xˢ factor: s = multiplicity of a+iβ as root. Usually s=0, but if the guess overlaps with yₕ, multiply by x."
                    },
                    "variation_of_parameters": {
                        "formula": "yₚ = y₁∫(−y₂g/W)dx + y₂∫(y₁g/W)dx",
                        "wronskian": "W = y₁y₂' − y₂y₁'",
                        "use_when": "Undetermined coefficients doesn't work (rhs is tan x, sec x, ln x, etc.)",
                        "hint": "More general than undetermined coefficients but messier. Know both methods."
                    }
                }
            },

            "laplace_transforms": {
                "definition": "L{f(t)} = F(s) = ∫₀^∞ e^(−st)f(t)dt",
                "key_transforms": [
                    "L{1} = 1/s",
                    "L{t^n} = n!/s^(n+1)",
                    "L{e^(at)} = 1/(s−a)",
                    "L{sin(bt)} = b/(s²+b²)",
                    "L{cos(bt)} = s/(s²+b²)",
                    "L{e^(at)f(t)} = F(s−a)  ← first shifting theorem",
                    "L{f'(t)} = sF(s) − f(0)",
                    "L{f''(t)} = s²F(s) − sf(0) − f'(0)",
                    "L{u(t−c)f(t−c)} = e^(−cs)F(s)  ← second shifting theorem"
                ],
                "method": "1. Take Laplace transform of ODE. 2. Solve algebraically for Y(s). 3. Inverse transform to get y(t).",
                "hint": "Laplace transforms convert ODEs into algebra. Great for initial value problems and discontinuous/impulsive forcing."
            },

            "systems_of_ODEs": {
                "form": "x' = Ax (linear system, constant coefficients)",
                "eigenvalue_method": {
                    "steps": "1. Find eigenvalues λ of A. 2. Find eigenvectors v. 3. General solution: x = Σ cₖ eˡᵏᵗ vₖ",
                    "cases": [
                        "Distinct real eigenvalues: straightforward",
                        "Complex eigenvalues α±βi: use Euler's formula, get sin/cos solutions",
                        "Repeated eigenvalues: may need generalized eigenvectors"
                    ],
                    "hint": "The eigenvalues of A determine stability: all negative real parts → stable (sink), positive → unstable (source), imaginary → center."
                },
                "phase_plane": {
                    "equilibrium_points": "Where x' = 0, y' = 0 simultaneously.",
                    "classification": [
                        "Eigenvalues both negative real → stable node (sink)",
                        "Eigenvalues both positive real → unstable node (source)",
                        "Eigenvalues opposite sign → saddle (unstable)",
                        "Complex eigenvalues, negative real part → stable spiral",
                        "Complex eigenvalues, positive real part → unstable spiral",
                        "Pure imaginary eigenvalues → center (neutrally stable)"
                    ]
                }
            },

            "nonlinear_ODEs": {
                "overview": "Most real systems are nonlinear. Exact solutions rarely exist. Use qualitative methods, linearization, and numerical methods.",
                "linearization": {
                    "method": "Near equilibrium (x₀,y₀): compute Jacobian J = [∂f/∂x, ∂f/∂y; ∂g/∂x, ∂g/∂y] at (x₀,y₀). Classify by eigenvalues of J.",
                    "hint": "Linearization is only valid near the equilibrium. Global behavior can be completely different."
                },
                "stability_lyapunov": {
                    "concept": "A Lyapunov function V(x) > 0 with dV/dt < 0 along trajectories proves stability without solving the ODE.",
                    "hint": "Think of V as an energy function. If energy is always decreasing, the system is stable. Finding V is an art."
                },
                "bifurcations": {
                    "saddle_node": "Two equilibria collide and annihilate.",
                    "pitchfork": "One equilibrium splits into three (common in symmetric systems).",
                    "hopf": "Equilibrium loses stability and a limit cycle is born.",
                    "hint": "Bifurcations are qualitative changes in system behavior as a parameter varies. They are phase transitions."
                },
                "chaos": {
                    "definition": "Deterministic system with sensitive dependence on initial conditions.",
                    "lorenz_system": "dx/dt = σ(y−x), dy/dt = x(ρ−z)−y, dz/dt = xy−βz. Canonical chaotic system. σ=10, ρ=28, β=8/3 gives the butterfly attractor.",
                    "lyapunov_exponent": "Positive Lyapunov exponent → chaotic. Measures rate of divergence of nearby trajectories.",
                    "hint": "Chaos is NOT randomness. It is deterministic but unpredictable beyond a finite horizon. The Lorenz attractor has fractal dimension ≈ 2.06."
                },
                "van_der_pol": {
                    "equation": "ẍ − μ(1−x²)ẋ + x = 0",
                    "behavior": "μ > 0: limit cycle. Small μ: nearly harmonic. Large μ: relaxation oscillations.",
                    "physical_meaning": "Models self-sustained oscillations (circuits, biological rhythms). The limit cycle is a stable attractor."
                },
                "duffing_oscillator": {
                    "equation": "ẍ + δẋ + αx + βx³ = γcos(ωt)",
                    "behavior": "Can exhibit period-doubling bifurcations and chaos. Double-well potential for α<0 shows complex dynamics."
                }
            },

            "partial_differential_equations": {
                "classification": {
                    "second_order": "Auₓₓ + Buₓᵧ + Cuᵧᵧ + ... = 0",
                    "discriminant": "Δ = B²−4AC: Δ<0 elliptic, Δ=0 parabolic, Δ>0 hyperbolic",
                    "examples": ["Laplace (elliptic): Δu=0", "Heat (parabolic): uₜ=αΔu", "Wave (hyperbolic): uₜₜ=c²Δu"]
                },
                "heat_equation": {
                    "form": "∂u/∂t = α²∂²u/∂x²",
                    "solution_separation": "u(x,t) = X(x)T(t). X''/X = T'/(α²T) = −λ². Leads to Fourier series.",
                    "hint": "Solutions decay exponentially in time. Heat spreads out and smooths — parabolic equations are smoothing."
                },
                "wave_equation": {
                    "form": "∂²u/∂t² = c²∂²u/∂x²",
                    "dalembert_solution": "u(x,t) = f(x+ct) + g(x−ct). Sum of left and right traveling waves.",
                    "hint": "Wave equation preserves discontinuities (they travel). Heat equation destroys them (smoothing)."
                },
                "laplaces_equation": {
                    "form": "∂²u/∂x² + ∂²u/∂y² = 0 (or Δu = 0 in higher dims)",
                    "solutions": "Harmonic functions. Solutions are the real parts of analytic complex functions.",
                    "maximum_principle": "Maximum and minimum of a harmonic function occur on the boundary.",
                    "hint": "Laplace equation describes steady-state (equilibrium) heat distribution, electrostatics, and fluid flow."
                }
            },

            "numerical_methods": [
                { "name": "Euler's Method", "formula": "yₙ₊₁ = yₙ + hf(xₙ, yₙ)", "error": "O(h) per step, O(h) global", "hint": "Simple but inaccurate. Good for understanding, not production." },
                { "name": "Runge-Kutta 4 (RK4)", "formula": "yₙ₊₁ = yₙ + h/6(k₁ + 2k₂ + 2k₃ + k₄)", "error": "O(h⁴) per step, O(h⁴) global", "hint": "The workhorse of numerical ODEs. Accurate, stable, and almost always the right choice." }
            ]
        }),
        "Differential equations: first order (separable, linear, exact, Bernoulli), second order, Laplace transforms, systems, nonlinear ODEs, chaos, bifurcations, PDEs, numerical methods",
        0.93,
    )
    .expect("math_differential_equations program")
}

// ── Vortex-Based Mathematics ──────────────────────────────────────────────────

pub fn vortex_mathematics_program() -> Program {
    Program::from_knowledge(
        Domain::Custom("math_vortex".into()),
        "1.0.0",
        &serde_json::json!({
            "domain_overview": "Vortex-Based Mathematics (VBM) — a numerical philosophy developed by Marko Rodin, championed by Randy Powell. Based on the properties of mod-9 arithmetic and a toroidal geometric model. Connected to Tesla's 3-6-9 insight and digital root theory.",

            "tesla_insight": {
                "quote": "If you only knew the magnificence of the 3, 6 and 9, then you would have a key to the universe. — Nikola Tesla",
                "meaning": "3, 6, and 9 never appear in the doubling/halving sequence. They form a separate axis of symmetry — the 'primal force' in VBM.",
                "tesla_369_pattern": "3→6→3→6... (×2 starting from 3: 3,6,12→3,6,3,6...)",
                "hint": "In mod-9 arithmetic: 3+3=6, 6+6=12→3, 3+3=6. The 3,6 axis oscillates. 9=0 in mod-9 (the void/infinity)."
            },

            "digital_root": {
                "definition": "The digital root of n is the single digit obtained by repeatedly summing digits until one digit remains.",
                "formula": "dr(n) = 1 + ((n−1) mod 9) for n ≥ 1. dr(0) = 0.",
                "properties": [
                    "dr(a+b) = dr(dr(a) + dr(b))",
                    "dr(a×b) = dr(dr(a) × dr(b))",
                    "dr(9) = 9, not 0 — 9 is the identity of digital roots",
                    "dr(n) = n mod 9, except dr(9k) = 9 not 0"
                ],
                "hint": "Digital roots are the arithmetic of mod-9 but with 9 as the identity instead of 0. This is why 9 behaves like infinity — it absorbs multiplication."
            },

            "doubling_sequence": {
                "sequence": "1, 2, 4, 8, 7, 5, 1, 2, 4, 8, 7, 5, ... (repeating period-6)",
                "derivation": "1×2=2, 2×2=4, 4×2=8, 8×2=16→7, 7×2=14→5, 5×2=10→1. Cycle returns.",
                "halving_sequence": "1, 5, 7, 8, 4, 2, 1, ... (reverse of doubling)",
                "together": "Doubling and halving create two complementary streams on the vortex: 1-2-4-8-7-5 and its mirror.",
                "excluded": "3, 6, and 9 never appear. They form the perpendicular axis.",
                "nine_point_circle": "Plot digits 1-9 around a circle. Connect doubling sequence: 1→2→4→8→7→5→1. Pattern never touches 3,6,9.",
                "hint": "The six-point star pattern (hexagram) formed by connecting 1-2-4-8-7-5 on the nine-point circle is the VBM 'winding number' — the signature of the torus."
            },

            "mod_9_arithmetic": {
                "multiplication_table_digital_roots": [
                    [1,2,3,4,5,6,7,8,9],
                    [2,4,6,8,1,3,5,7,9],
                    [3,6,9,3,6,9,3,6,9],
                    [4,8,3,7,2,6,1,5,9],
                    [5,1,6,2,7,3,8,4,9],
                    [6,3,9,6,3,9,6,3,9],
                    [7,5,3,1,8,6,4,2,9],
                    [8,7,6,5,4,3,2,1,9],
                    [9,9,9,9,9,9,9,9,9]
                ],
                "observations": [
                    "Row 9 (and column 9): all 9s. Nine absorbs everything.",
                    "Row 3 and 6: 3,6,9,3,6,9... — they only produce 3,6,9.",
                    "The 6×6 submatrix of rows/cols 1,2,4,5,7,8 is a closed group under multiplication mod 9.",
                    "This closed subgroup is isomorphic to ℤ/6 — the cyclic group of order 6."
                ]
            },

            "toroidal_model": {
                "concept": "VBM maps the number wheel (1–9) onto the surface of a torus. The doubling sequence traces the outer flow; the 3-6-9 axis is the central channel (the hole of the torus).",
                "poloidal_flow": "Around the tube of the torus. Carries 1-2-4-8-7-5 sequence.",
                "toroidal_flow": "Around the hole of the torus. Carries 3-6-9 oscillation.",
                "vortex": "The hole of the torus — where energy concentrates infinitely. 9 is the gateway.",
                "hint": "The torus is the only geometric shape that can rotate in two independent directions simultaneously. VBM claims this is the fundamental topology of energy flow in nature."
            },

            "fibonacci_digital_roots": {
                "sequence": "1,1,2,3,5,8,4,3,7,1,8,9,8,8,7,6,4,1,5,6,2,8,1,9,1,1,2,...",
                "period": "24 (the Pisano period for mod 9)",
                "pattern": "The digital roots of Fibonacci numbers cycle with period 24.",
                "nine_points": "9 appears at positions 12 and 24 — exactly at the half and end of each cycle.",
                "sum_of_period": "1+1+2+3+5+8+4+3+7+1+8+9+8+8+7+6+4+1+5+6+2+8+1+9 = 108 = 12×9",
                "hint": "108 = 4×27 = 4×3³. The number 108 appears throughout sacred geometry (108 beads, 108 degrees, etc.). Digital root of 108 = 9."
            },

            "connection_to_primes": {
                "observation": "All prime numbers greater than 3 have digital root 1, 2, 4, 5, 7, or 8 — never 3, 6, or 9.",
                "why": "Any number with digital root 3, 6, or 9 is divisible by 3 (and thus not prime unless the number itself is 3).",
                "form": "All primes > 3 are of the form 6k±1. Their digital roots cycle within {1,2,4,5,7,8}.",
                "hint": "This is rigorous: if dr(n) ∈ {3,6,9} then 3|n. The VBM observation about primes is mathematically correct."
            },

            "connection_to_master_equation": {
                "note": "Vortex mathematics describes a discrete analog of the holonomy framework in MASTER_EQUATION.md.",
                "correspondence": [
                    "The nine-point circle ↔ the constitutional manifold V_m(ℝ^N)",
                    "The doubling sequence (1-2-4-8-7-5) ↔ the L-type holonomy component (SO^+(m))",
                    "The 3-6-9 axis ↔ the Yettragrammaton gauge axis (the fixed point structure)",
                    "Digital root 9 (absorbs all multiplication) ↔ the identity element g of the holonomy group",
                    "Period-6 doubling cycle ↔ the Z/6 structure of the fundamental group π_1(V_m(ℝ^N)) modulo its 2-torsion",
                    "The torus topology ↔ the fiber bundle structure π: P → V_m(ℝ^N)"
                ],
                "caveat": "These correspondences are structural/analogical. The formal bridge between discrete VBM and the continuous holonomy framework is an open research question."
            },

            "marko_rodin_coil": {
                "description": "A toroidal coil wound according to the VBM doubling sequence. Rodin claims it produces anomalous efficiency.",
                "winding_pattern": "Wires follow the 1-2-4-8-7-5 path on the torus surface, never crossing the 3-6-9 axis.",
                "mathematical_basis": "The winding corresponds to a specific geodesic on the torus — one that avoids the singular axis.",
                "status": "The anomalous efficiency claims have not been independently replicated. The mathematical structure is real; the physical claims are unverified.",
                "hint": "Separate the mathematical structure (toroidal winding, mod-9 symmetry, geodesics) from the physical claims. The former is rigorous; the latter is speculative."
            },

            "432_hz_and_music": {
                "claim": "432 Hz tuning is mathematically harmonious with natural law.",
                "digital_root_432": "4+3+2 = 9",
                "digital_root_standard_440": "4+4+0 = 8",
                "digital_root_528": "5+2+8 = 15 → 6 (the 'love frequency' of solfeggio)",
                "octave_digital_roots": "432×2=864 (dr=9), 432×4=1728 (dr=9), all octaves of 432 have dr=9",
                "hint": "The digital root property of 432 Hz (dr=9) and its octaves is a genuine mathematical fact. The physical/biological claims require independent verification."
            }
        }),
        "Vortex-Based Mathematics: Tesla 3-6-9, digital roots, doubling sequence, mod-9 arithmetic, toroidal model, Fibonacci digital roots, connection to master equation",
        0.88,
    )
    .expect("math_vortex program")
}

// ── Advanced Mathematics ──────────────────────────────────────────────────────

pub fn advanced_mathematics_program() -> Program {
    Program::from_knowledge(
        Domain::Custom("math_advanced".into()),
        "1.0.0",
        &serde_json::json!({
            "domain_overview": "Advanced mathematics: linear algebra, abstract algebra, number theory, complex analysis, topology, real analysis, probability, and information theory.",

            "linear_algebra": {
                "vector_spaces": "A vector space V over field F satisfies 8 axioms (closure under + and scalar mult, distributivity, associativity, identity, inverse, etc.).",
                "subspace": "A subset W ⊆ V is a subspace iff: 0 ∈ W, closed under + and scalar mult.",
                "basis": "A linearly independent spanning set. Every vector has a unique representation in terms of basis vectors.",
                "dimension": "Number of basis vectors. dim(V) is well-defined (all bases have the same size).",
                "linear_transformation": "T: V → W is linear iff T(au+bv) = aT(u) + bT(v).",
                "null_space_range": "Null(T) = {v: T(v)=0}, Range(T) = {T(v): v∈V}. Rank-nullity: dim(V) = dim(Null) + dim(Range).",
                "eigenvalues_eigenvectors": {
                    "definition": "Av = λv. v ≠ 0 is eigenvector with eigenvalue λ.",
                    "characteristic_polynomial": "det(A − λI) = 0. Roots are eigenvalues.",
                    "diagonalization": "A = PDP⁻¹ where D diagonal (eigenvalues), P columns = eigenvectors. Requires n linearly independent eigenvectors.",
                    "hint": "Eigenvalues are the 'natural frequencies' of the linear map. They reveal how the transformation stretches/compresses space."
                },
                "svd": {
                    "theorem": "Every matrix A = UΣVᵀ where U,V orthogonal, Σ diagonal with non-negative singular values σᵢ = √(eigenvalues of AᵀA).",
                    "applications": "Principal component analysis, low-rank approximation, pseudoinverse, image compression.",
                    "hint": "SVD is the most important matrix decomposition. It works for ANY matrix, not just square ones. The singular values measure 'how much' A stretches each direction."
                },
                "inner_product_spaces": "⟨u,v⟩ satisfying: linearity, conjugate symmetry, positive definiteness. Induces norm ‖v‖ = √⟨v,v⟩.",
                "gram_schmidt": "Converts any basis into an orthonormal basis. Project out components iteratively.",
                "spectral_theorem": "A symmetric matrix A = QΛQᵀ with real eigenvalues and orthogonal eigenvectors. The Stiefel manifold V_m(ℝ^N) parameterizes such decompositions."
            },

            "abstract_algebra": {
                "groups": {
                    "definition": "Set G with operation · satisfying: closure, associativity, identity (e), inverse.",
                    "abelian": "Commutative group: a·b = b·a.",
                    "order": "Order of group = |G|. Order of element a = smallest n with aⁿ = e.",
                    "subgroup": "H ≤ G: H non-empty, closed under · and inverses.",
                    "lagrange_theorem": "|H| divides |G|. Order of element divides |G|.",
                    "isomorphism_theorems": "First: G/Ker(φ) ≅ Im(φ). Fundamental theorem connecting quotients to homomorphisms.",
                    "key_groups": ["ℤₙ (integers mod n)", "Sₙ (permutations on n elements)", "Dₙ (symmetries of regular n-gon)", "GL(n,F) (invertible n×n matrices)", "SO(n) (rotation group — appears in master equation)"]
                },
                "rings_and_fields": {
                    "ring": "Set R with + (abelian group) and × (associative, distributive over +). May lack × identity or commutativity.",
                    "field": "Commutative ring where every nonzero element has multiplicative inverse. ℚ, ℝ, ℂ, ℤₚ (p prime) are fields.",
                    "hint": "Fields are where most of mathematics lives comfortably. Rings are more general but harder to work with."
                },
                "galois_theory": {
                    "main_theorem": "There is a bijection between subgroups of the Galois group Gal(L/K) and intermediate fields K ⊆ F ⊆ L.",
                    "applications": "Proves quintic equations have no general radical solution. Proves impossibility of trisecting angle, squaring circle, duplicating cube with compass and straightedge.",
                    "hint": "Galois theory translates geometric impossibility into group-theoretic impossibility. One of the deepest ideas in mathematics."
                }
            },

            "number_theory": {
                "divisibility": "a|b means b = ak for some integer k.",
                "gcd_lcm": "gcd(a,b): largest divisor of both. lcm(a,b) = ab/gcd(a,b). Euclidean algorithm computes gcd.",
                "prime_fundamental_theorem": "Every integer > 1 has a unique factorization into primes.",
                "modular_arithmetic": {
                    "definition": "a ≡ b (mod n) iff n|(a−b).",
                    "fermat_little_theorem": "aᵖ ≡ a (mod p) for prime p. If gcd(a,p)=1: a^(p−1) ≡ 1 (mod p).",
                    "euler_theorem": "a^φ(n) ≡ 1 (mod n) when gcd(a,n)=1. φ(n) = Euler's totient.",
                    "crt": "Chinese Remainder Theorem: if n₁,...,nₖ pairwise coprime, system x ≡ aᵢ (mod nᵢ) has unique solution mod n₁···nₖ.",
                    "hint": "Modular arithmetic is the foundation of cryptography. RSA uses Fermat's little theorem at its core."
                },
                "quadratic_reciprocity": "The crown jewel of classical number theory. (p/q)(q/p) = (−1)^((p−1)/2·(q−1)/2) for odd primes p,q.",
                "prime_distribution": {
                    "prime_number_theorem": "π(x) ~ x/ln(x). The number of primes ≤ x grows as x/ln(x).",
                    "riemann_zeta": "ζ(s) = Σ n^(−s) = Π (1−p^(−s))^(−1). Zeros encode prime distribution.",
                    "hint": "The Riemann Hypothesis (if true) gives the most precise possible error term for the prime counting function."
                }
            },

            "complex_analysis": {
                "analytic_functions": {
                    "definition": "f is analytic (holomorphic) at z₀ if it has a complex derivative there.",
                    "cauchy_riemann": "f = u+iv is analytic iff ∂u/∂x = ∂v/∂y and ∂u/∂y = −∂v/∂x.",
                    "hint": "The Cauchy-Riemann equations are enormously restrictive. Analytic functions are extremely rigid — knowing one tiny piece determines the whole function."
                },
                "contour_integration": {
                    "cauchy_integral_theorem": "∮_C f(z)dz = 0 if f analytic inside C.",
                    "cauchy_integral_formula": "f(z₀) = (1/2πi) ∮_C f(z)/(z−z₀)dz. A function's value at any interior point is determined by its boundary values.",
                    "residue_theorem": "∮_C f(z)dz = 2πi Σ Res(f, aₖ) where sum is over poles inside C.",
                    "residue_at_simple_pole": "Res(f, a) = lim_{z→a} (z−a)f(z)",
                    "hint": "The residue theorem converts contour integrals to algebraic computation. Immensely powerful for evaluating real integrals that resist elementary methods."
                },
                "conformal_maps": "Analytic functions with nonzero derivative preserve angles. Used to solve 2D Laplace equation by mapping complex domains.",
                "connections_to_master_equation": "The winding number ω in the master equation is the residue of 1/z at the origin — the simplest residue computation. Holomorphic functions and holonomy are deeply linked."
            },

            "topology": {
                "metric_spaces": "Distance function d satisfying: d(x,y)≥0, d(x,y)=0 iff x=y, symmetry, triangle inequality.",
                "continuity": "f is continuous at x₀ iff ∀ε>0 ∃δ>0: d(x,x₀)<δ ⟹ d(f(x),f(x₀))<ε. Open set definition: preimage of open set is open.",
                "compactness": "A space is compact if every open cover has a finite subcover. In ℝⁿ: compact iff closed and bounded (Heine-Borel).",
                "connectedness": "A space is connected if it cannot be split into two disjoint open sets. Simply connected: connected and every loop contracts to a point.",
                "fundamental_group": "π₁(X, x₀): group of homotopy classes of loops based at x₀. Measures 'holes' in the space. π₁(S¹) = ℤ (winding numbers!). π₁(V_m(ℝ^N)) = ℤ/2.",
                "manifolds": "Locally looks like ℝⁿ. Examples: spheres Sⁿ, tori Tⁿ, Stiefel manifolds V_m(ℝ^N).",
                "fiber_bundles": "π: E → B with fibers F = π⁻¹(b). Local triviality: locally E ≅ B×F. Holonomy measures global non-triviality.",
                "hint": "Topology is the study of properties preserved under continuous deformation. A coffee cup and a donut are topologically identical — both have one hole."
            },

            "information_theory": {
                "shannon_entropy": {
                    "formula": "H(X) = −Σᵢ p(xᵢ) log₂ p(xᵢ)",
                    "interpretation": "Average number of bits needed to encode outcomes of X. Maximum for uniform distribution.",
                    "maximum": "H(X) ≤ log₂ n for n outcomes, with equality iff uniform.",
                    "hint": "Entropy measures uncertainty/surprise. High entropy = unpredictable. Low entropy = predictable."
                },
                "mutual_information": "I(X;Y) = H(X) + H(Y) − H(X,Y) = H(X) − H(X|Y). Reduction in uncertainty about X given Y.",
                "kl_divergence": "D_KL(P‖Q) = Σ p(x) log(p(x)/q(x)). Not symmetric. Measures how P differs from Q.",
                "data_processing_inequality": "If X → Y → Z is a Markov chain, then I(X;Z) ≤ I(X;Y). Processing cannot increase information.",
                "channel_capacity": "C = max_{p(x)} I(X;Y). Maximum rate at which information can be reliably transmitted.",
                "connection_to_master_equation": "The ADCCL threshold θ_opt = 1 − H(R)/H(Ψ) is derived from the Data Processing Inequality applied to the constitutional projection channel."
            },

            "probability": {
                "axioms": ["P(Ω) = 1", "P(A) ≥ 0", "P(A∪B) = P(A)+P(B) for disjoint A,B"],
                "bayes_theorem": "P(A|B) = P(B|A)P(A)/P(B). Fundamental tool for updating beliefs given evidence.",
                "distributions": [
                    { "name": "Binomial", "pmf": "C(n,k)pᵏ(1−p)^(n−k)", "mean": "np", "var": "np(1−p)" },
                    { "name": "Poisson", "pmf": "λᵏe^(−λ)/k!", "mean": "λ", "var": "λ", "hint": "Limit of binomial as n→∞, p→0, np=λ. Models rare events." },
                    { "name": "Normal", "pdf": "(1/σ√(2π))exp(−(x−μ)²/2σ²)", "mean": "μ", "var": "σ²", "hint": "68-95-99.7 rule: 1,2,3 standard deviations contain 68%, 95%, 99.7% of mass." },
                    { "name": "Exponential", "pdf": "λe^(−λx)", "mean": "1/λ", "var": "1/λ²", "hint": "Memoryless: P(X>s+t|X>s) = P(X>t). Models waiting times." }
                ],
                "law_of_large_numbers": "Sample mean converges to expected value as n→∞.",
                "central_limit_theorem": "Sum of n i.i.d. random variables (finite variance) converges in distribution to Normal as n→∞. One of the most important theorems in all of mathematics.",
                "hint": "CLT explains why the normal distribution appears everywhere. It is not assumed — it is derived."
            }
        }),
        "Advanced mathematics: linear algebra (SVD, eigenvalues, Gram-Schmidt), abstract algebra, number theory, complex analysis (residues, contour integration), topology (fiber bundles, fundamental group), information theory (Shannon entropy, DPI), probability (CLT, Bayes)",
        0.95,
    )
    .expect("math_advanced program")
}

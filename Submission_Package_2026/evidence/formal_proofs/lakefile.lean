import Lake
open Lake DSL

package «ChyrenLogic» where
  -- Settings applied to both it and its dependencies
  leanOptions := #[
    ⟨`pp.unicode.fun, true⟩ -- [pretty printer] [unicode] [functions]
  ]

lean_lib «ChyrenLogic» where
  -- add library configuration options here

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git"

@[default_target]
lean_exe «chyren_logic» where
  root := `Main

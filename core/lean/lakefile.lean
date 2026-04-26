import Lake
open Lake DSL

package q5 where

lean_lib Q5 where
  roots := #[`Q5]

@[default_target]
lean_exe q5check where
  root := `Q5.Main

module Test.Main where

import Prelude

import Data.Maybe (Maybe(..))
import Effect (Effect)
import Effect.Aff (Milliseconds(..), delay, launchAff_)
import Test.Spec (it)
import Test.Spec.Assertions (shouldEqual)
import Test.Spec.Config (defaultConfig)
import Test.Spec.Reporter (specReporter)
import Test.Spec.Runner (runSpecPure')

main :: Effect Unit
main = launchAff_ $ runSpecPure' config [specReporter] do
  it "passes quickly" $
    5 `shouldEqual` (3 + 2)

  -- Keep a wide margin between the timeout and the delayed test: with a tight
  -- margin (10 ms vs 15 ms) any scheduling jitter under load lets the delayed
  -- test win the race and the timeout never fires.
  it "times out" $
    delay (Milliseconds 500.0)

  it "shouldn't get to run" $
    2 `shouldEqual` 3

  where
    config = defaultConfig { failFast = true, timeout = Just $ Milliseconds 50.0, exit = false }

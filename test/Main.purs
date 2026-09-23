module Test.Main where

import Prelude

import Data.Array (fold)
import Data.Maybe (Maybe(..))
import Effect (Effect)
import Effect.Aff (Milliseconds(..), launchAff_)
import Effect.Class (liftEffect)
import Options.Applicative as Opt
import Record (merge)
import Test.Integration (integrationSpecs)
import Test.Spec.AssertionSpec (assertionSpec)
import Test.Spec.HoistSpec (hoistSpecSpec)
import Test.Spec.HookSpec (hookSpec)
import Test.Spec.ParallelSpec (parallelSpec)
import Test.Spec.Reporter (specReporter)
import Test.Spec.Reporter.TeamCitySpec (teamcitySpec)
import Test.Spec.Runner.Node (runSpecAndExitProcess')
import Test.Spec.Runner.Node.Config as Config
import Test.Spec.RunnerSpec (runnerSpec)

main :: Effect Unit
main = launchAff_ do
  config <- liftEffect $
    Config.fromCommandLine' defaultConfig (Config.commandLineOptionParsers <> [debug, accept, integrationFlag])
    <#> _ { timeout = Just $ Milliseconds 30000.0 }
  -- The integration suite shells out to `npm`/`npx`/`spago` in a temporary
  -- environment. The native port cannot run that toolchain, so it is opt-in.
  integration <-
    if config.integration then
      integrationSpecs { debug: config.debug, accept: config.accept }
    else
      pure (pure unit)
  liftEffect $
    runSpecAndExitProcess'
      { defaultConfig: config
      , parseCLIOptions: false
      }
      [specReporter] $
      pureSpecs *> integration
  where
    pureSpecs = do
      runnerSpec
      assertionSpec
      hookSpec
      hoistSpecSpec
      parallelSpec
      teamcitySpec

type Config = Config.TestRunConfig' (debug :: Boolean, accept :: Boolean, integration :: Boolean)

defaultConfig :: Config
defaultConfig = Config.defaultConfig `merge` { debug: false, accept: false, integration: false }

debug :: Config.OptionParser Config
debug = ado
  d <- Opt.switch $ fold
    [ Opt.long "debug"
    , Opt.help "Do not destroy temporary directory used for integration tests."
    ]

  in _ { debug = d }

accept :: Config.OptionParser Config
accept = ado
  a <- Opt.switch $ fold
    [ Opt.long "accept"
    , Opt.help "Accept all test outputs as new expected outputs."
    ]

  in _ { accept = a }

integrationFlag :: Config.OptionParser Config
integrationFlag = ado
  i <- Opt.switch $ fold
    [ Opt.long "integration"
    , Opt.help "Also run the integration tests (requires node, npm and npx)."
    ]

  in _ { integration = i }

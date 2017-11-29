#!/usr/bin/env stack
-- stack runghc --resolver nightly-2017-11-28 --package shake --install-ghc

import           Data.Maybe        (fromMaybe)
import           Data.Monoid
import           Development.Shake
import           System.Exit       (ExitCode (..))

main :: IO ()
main = shakeArgs shakeOptions { shakeFiles=".shake" } $ do
    want [ "target/{{ project }}" ]

    "target/{{ project }}" %> \_ -> do
        dats <- getDirectoryFiles "" ["//*.dats"]
        sats <- getDirectoryFiles "" ["//*.sats"]
        hats <- getDirectoryFiles "" ["//*.hats"]
        cats <- getDirectoryFiles "" ["//*.cats"]
        need $ dats <> sats <> hats <> cats
        cmd_ ["mkdir", "-p", "target"]
        let patshome = "/usr/local/lib/ats2-postiats-0.3.8"
        (Exit c, Stderr err) <- command [EchoStderr False, AddEnv "PATSHOME" patshome] "patscc" (dats ++ ["-DATS_MEMALLOC_LIBC", "-o", "target/{{ project }}", "-cleanaft", "-O2", "-mtune=native"])
        cmd_ [Stdin err] Shell "pats-filter"
        if c /= ExitSuccess
            then error "patscc failure"
            else pure ()

    "install" ~> do
        need ["target/{{ project }}"]
        home <- getEnv "HOME"
        cmd ["cp", "target/{{ project }}", fromMaybe "" home ++ "/.local/bin"]

    "run" ~> do
        need ["target/{{ project }}"]
        cmd ["./target/{{ project }}", "."]

    "clean" ~> do
        cmd_ ["sn", "c"]
        removeFilesAfter "." ["//*.c", "tags", "build"]
        removeFilesAfter ".shake" ["//*"]
        removeFilesAfter "target" ["//*"]

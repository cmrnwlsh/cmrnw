#!/usr/bin/env nu

const p = path self | path dirname

export def main [--dev] {
  let client = if $dev { build client --dev } else { build client }
  let server = if $dev { build server --dev } else { build server }  
  for job in [$client, $server] {
    job recv | print
  }
}

export def 'build client' [--dev] {
  job spawn {
    let _ = (ls ($p)/client | where type == dir).name | par-each { |$c|
      let c = $c | path basename
      let flags = if $dev { ["--dev"] } else { [] }
      wasm-pack build ...$flags --out-dir ($p)/static/($c) ($p)/client/($c)
      $'(ansi c)($c)(ansi rst): build complete' | job send 0
    }
  }
}

export def 'build server' [--dev] {
  job spawn {
    let flags = if $dev { [] } else { ["--release"] }
    cargo build --manifest-path ($p)/Cargo.toml --package server ...$flags 
    $'(ansi c)server(ansi rst): build complete' | job send 0
  }
}

export def 'build clean' [] {
  cargo clean
  let len = if (ls ($p)/static | length) == 0 { 0 } else {
    ls (($p)/static/**/* | into glob) | where type == file | length
  }
  rm -rf (($p)/static/* | into glob)
  print $'     (ansi gb)Removed(ansi rst) ($len) static files'
}

export def --wrapped 'build run' [...args] {
  build
  cargo run ...$args --package server
}

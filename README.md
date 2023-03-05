# matter-rs build issues with ESP32 (RISC-V)

This is a hopefully minimal repo that reproduces the build issues with matter-rs for ESP32 RISC-V targets.
The xtensa targets might be an issue, I however encountered the issues while building for an ESP32-C3.

To reproduce, it might be sufficient to run `cargo build` assuming you have the `riscv32imc-esp-espidf` Rust target.

A sample of errors is in [build-output.log](./build-output.log).

The dependency tree looks like:

```
esp32-c3-matter-minimal v0.1.0 (/home/user/esp32-c3-matter-minimal)
├── esp-idf-sys v0.32.1
│   ├── build-time v0.1.2 (proc-macro)
│   │   ├── chrono v0.4.23
│   │   │   ├── iana-time-zone v0.1.53
│   │   │   │   └── core-foundation-sys v0.8.3
│   │   │   ├── num-integer v0.1.45
│   │   │   │   └── num-traits v0.2.15
│   │   │   │       [build-dependencies]
│   │   │   │       └── autocfg v1.1.0
│   │   │   │   [build-dependencies]
│   │   │   │   └── autocfg v1.1.0
│   │   │   ├── num-traits v0.2.15 (*)
│   │   │   └── time v0.1.45
│   │   │       └── libc v0.2.139
│   │   ├── once_cell v1.17.1
│   │   ├── proc-macro2 v1.0.51
│   │   │   └── unicode-ident v1.0.7
│   │   ├── quote v1.0.23
│   │   │   └── proc-macro2 v1.0.51 (*)
│   │   └── syn v1.0.109
│   │       ├── proc-macro2 v1.0.51 (*)
│   │       ├── quote v1.0.23 (*)
│   │       └── unicode-ident v1.0.7
│   ├── const_format v0.2.30
│   │   └── const_format_proc_macros v0.2.29 (proc-macro)
│   │       ├── proc-macro2 v1.0.51 (*)
│   │       ├── quote v1.0.23 (*)
│   │       └── unicode-xid v0.2.4
│   └── libc v0.2.139
│   [build-dependencies]
│   ├── anyhow v1.0.69
│   ├── bindgen v0.63.0
│   │   ├── bitflags v1.3.2
│   │   ├── cexpr v0.6.0
│   │   │   └── nom v7.1.3
│   │   │       ├── memchr v2.5.0
│   │   │       └── minimal-lexical v0.2.1
│   │   ├── clang-sys v1.6.0
│   │   │   ├── glob v0.3.1
│   │   │   ├── libc v0.2.139
│   │   │   └── libloading v0.7.4
│   │   │       └── cfg-if v1.0.0
│   │   │   [build-dependencies]
│   │   │   └── glob v0.3.1
│   │   ├── lazy_static v1.4.0
│   │   ├── lazycell v1.3.0
│   │   ├── log v0.4.17
│   │   │   └── cfg-if v1.0.0
│   │   ├── peeking_take_while v0.1.2
│   │   ├── proc-macro2 v1.0.51 (*)
│   │   ├── quote v1.0.23 (*)
│   │   ├── regex v1.7.1
│   │   │   ├── aho-corasick v0.7.20
│   │   │   │   └── memchr v2.5.0
│   │   │   ├── memchr v2.5.0
│   │   │   └── regex-syntax v0.6.28
│   │   ├── rustc-hash v1.1.0
│   │   ├── shlex v1.1.0
│   │   ├── syn v1.0.109 (*)
│   │   └── which v4.4.0
│   │       ├── either v1.8.1
│   │       └── libc v0.2.139
│   ├── cargo_metadata v0.15.3
│   │   ├── camino v1.1.3
│   │   │   └── serde v1.0.152
│   │   │       └── serde_derive v1.0.152 (proc-macro)
│   │   │           ├── proc-macro2 v1.0.51 (*)
│   │   │           ├── quote v1.0.23 (*)
│   │   │           └── syn v1.0.109 (*)
│   │   ├── cargo-platform v0.1.2
│   │   │   └── serde v1.0.152 (*)
│   │   ├── semver v1.0.16
│   │   │   └── serde v1.0.152 (*)
│   │   ├── serde v1.0.152 (*)
│   │   ├── serde_json v1.0.93
│   │   │   ├── itoa v1.0.6
│   │   │   ├── ryu v1.0.13
│   │   │   └── serde v1.0.152 (*)
│   │   └── thiserror v1.0.38
│   │       └── thiserror-impl v1.0.38 (proc-macro)
│   │           ├── proc-macro2 v1.0.51 (*)
│   │           ├── quote v1.0.23 (*)
│   │           └── syn v1.0.109 (*)
│   ├── embuild v0.31.0
│   │   ├── anyhow v1.0.69
│   │   ├── bindgen v0.63.0 (*)
│   │   ├── bitflags v1.3.2
│   │   ├── cmake v0.1.49
│   │   │   └── cc v1.0.79
│   │   ├── dirs v4.0.0
│   │   │   └── dirs-sys v0.3.7
│   │   │       └── libc v0.2.139
│   │   ├── filetime v0.2.20
│   │   │   ├── cfg-if v1.0.0
│   │   │   └── libc v0.2.139
│   │   ├── globwalk v0.8.1
│   │   │   ├── bitflags v1.3.2
│   │   │   ├── ignore v0.4.20
│   │   │   │   ├── globset v0.4.10
│   │   │   │   │   ├── aho-corasick v0.7.20 (*)
│   │   │   │   │   ├── bstr v1.3.0
│   │   │   │   │   │   └── memchr v2.5.0
│   │   │   │   │   ├── fnv v1.0.7
│   │   │   │   │   ├── log v0.4.17 (*)
│   │   │   │   │   └── regex v1.7.1 (*)
│   │   │   │   ├── lazy_static v1.4.0
│   │   │   │   ├── log v0.4.17 (*)
│   │   │   │   ├── memchr v2.5.0
│   │   │   │   ├── regex v1.7.1 (*)
│   │   │   │   ├── same-file v1.0.6
│   │   │   │   ├── thread_local v1.1.7
│   │   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   │   └── once_cell v1.17.1
│   │   │   │   └── walkdir v2.3.2
│   │   │   │       └── same-file v1.0.6
│   │   │   └── walkdir v2.3.2 (*)
│   │   ├── log v0.4.17 (*)
│   │   ├── remove_dir_all v0.7.0
│   │   │   └── libc v0.2.139
│   │   ├── serde v1.0.152 (*)
│   │   ├── serde_json v1.0.93 (*)
│   │   ├── shlex v1.1.0
│   │   ├── strum v0.24.1
│   │   │   └── strum_macros v0.24.3 (proc-macro)
│   │   │       ├── heck v0.4.1
│   │   │       ├── proc-macro2 v1.0.51 (*)
│   │   │       ├── quote v1.0.23 (*)
│   │   │       ├── rustversion v1.0.11 (proc-macro)
│   │   │       └── syn v1.0.109 (*)
│   │   ├── tempfile v3.4.0
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── fastrand v1.9.0
│   │   │   └── rustix v0.36.9
│   │   │       ├── bitflags v1.3.2
│   │   │       ├── errno v0.2.8
│   │   │       │   └── libc v0.2.139
│   │   │       ├── io-lifetimes v1.0.5
│   │   │       │   └── libc v0.2.139
│   │   │       └── libc v0.2.139
│   │   ├── thiserror v1.0.38 (*)
│   │   └── which v4.4.0 (*)
│   ├── envy v0.4.2
│   │   └── serde v1.0.152 (*)
│   ├── regex v1.7.1 (*)
│   ├── serde v1.0.152 (*)
│   └── strum v0.24.1 (*)
└── matter-iot v0.1.0 (https://github.com/project-chip/matter-rs?branch=main#05263e7a)
    ├── async-channel v1.8.0
    │   ├── concurrent-queue v2.1.0
    │   │   └── crossbeam-utils v0.8.15
    │   │       └── cfg-if v1.0.0
    │   ├── event-listener v2.5.3
    │   └── futures-core v0.3.26
    ├── bitflags v1.3.2
    ├── boxslab v0.1.0 (https://github.com/project-chip/matter-rs?branch=main#05263e7a)
    │   └── bitmaps v3.2.0
    ├── byteorder v1.4.3
    ├── chrono v0.4.23
    │   ├── iana-time-zone v0.1.53
    │   ├── num-integer v0.1.45
    │   │   └── num-traits v0.2.15
    │   │       [build-dependencies]
    │   │       └── autocfg v1.1.0
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   └── num-traits v0.2.15 (*)
    ├── colored v2.0.0
    │   ├── atty v0.2.14
    │   │   └── libc v0.2.139
    │   └── lazy_static v1.4.0
    ├── env_logger v0.10.0
    │   ├── humantime v2.1.0
    │   ├── is-terminal v0.4.4
    │   │   ├── io-lifetimes v1.0.5
    │   │   │   └── libc v0.2.139
    │   │   └── rustix v0.36.9
    │   │       ├── bitflags v1.3.2
    │   │       ├── errno v0.2.8
    │   │       │   └── libc v0.2.139
    │   │       ├── io-lifetimes v1.0.5 (*)
    │   │       └── libc v0.2.139
    │   ├── log v0.4.17
    │   │   └── cfg-if v1.0.0
    │   ├── regex v1.7.1
    │   │   ├── aho-corasick v0.7.20
    │   │   │   └── memchr v2.5.0
    │   │   ├── memchr v2.5.0
    │   │   └── regex-syntax v0.6.28
    │   └── termcolor v1.2.0
    ├── esp-idf-sys v0.32.1 (*)
    ├── generic-array v0.14.6
    │   └── typenum v1.16.0
    │   [build-dependencies]
    │   └── version_check v0.9.4
    ├── heapless v0.7.16
    │   ├── hash32 v0.2.1
    │   │   └── byteorder v1.4.3
    │   └── stable_deref_trait v1.2.0
    │   [build-dependencies]
    │   └── rustc_version v0.4.0
    │       └── semver v1.0.16 (*)
    ├── log v0.4.17 (*)
    ├── matter_macro_derive v0.1.0 (proc-macro) (https://github.com/project-chip/matter-rs?branch=main#05263e7a)
    │   ├── proc-macro2 v1.0.51 (*)
    │   ├── quote v1.0.23 (*)
    │   └── syn v1.0.109 (*)
    ├── num v0.4.0
    │   ├── num-bigint v0.4.3
    │   │   ├── num-integer v0.1.45 (*)
    │   │   └── num-traits v0.2.15 (*)
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   ├── num-complex v0.4.3
    │   │   └── num-traits v0.2.15 (*)
    │   ├── num-integer v0.1.45 (*)
    │   ├── num-iter v0.1.43
    │   │   ├── num-integer v0.1.45 (*)
    │   │   └── num-traits v0.2.15 (*)
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   ├── num-rational v0.4.1
    │   │   ├── num-bigint v0.4.3 (*)
    │   │   ├── num-integer v0.1.45 (*)
    │   │   └── num-traits v0.2.15 (*)
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   └── num-traits v0.2.15 (*)
    ├── num-derive v0.3.3 (proc-macro)
    │   ├── proc-macro2 v1.0.51 (*)
    │   ├── quote v1.0.23 (*)
    │   └── syn v1.0.109 (*)
    ├── num-traits v0.2.15 (*)
    ├── owning_ref v0.4.1
    │   └── stable_deref_trait v1.2.0
    ├── qrcode v0.12.0
    │   └── checked_int_cast v1.0.0
    ├── rand v0.8.5
    │   ├── libc v0.2.139
    │   ├── rand_chacha v0.3.1
    │   │   ├── ppv-lite86 v0.2.17
    │   │   └── rand_core v0.6.4
    │   │       └── getrandom v0.2.8
    │   │           ├── cfg-if v1.0.0
    │   │           └── libc v0.2.139
    │   └── rand_core v0.6.4 (*)
    ├── safemem v0.3.3
    ├── smol v1.3.0
    │   ├── async-channel v1.8.0 (*)
    │   ├── async-executor v1.5.0
    │   │   ├── async-lock v2.7.0
    │   │   │   └── event-listener v2.5.3
    │   │   ├── async-task v4.3.0
    │   │   ├── concurrent-queue v2.1.0 (*)
    │   │   ├── fastrand v1.9.0
    │   │   ├── futures-lite v1.12.0
    │   │   │   ├── fastrand v1.9.0
    │   │   │   ├── futures-core v0.3.26
    │   │   │   ├── futures-io v0.3.26
    │   │   │   ├── memchr v2.5.0
    │   │   │   ├── parking v2.0.0
    │   │   │   ├── pin-project-lite v0.2.9
    │   │   │   └── waker-fn v1.1.0
    │   │   └── slab v0.4.8
    │   │       [build-dependencies]
    │   │       └── autocfg v1.1.0
    │   ├── async-fs v1.6.0
    │   │   ├── async-lock v2.7.0 (*)
    │   │   ├── blocking v1.3.0
    │   │   │   ├── async-channel v1.8.0 (*)
    │   │   │   ├── async-lock v2.7.0 (*)
    │   │   │   ├── async-task v4.3.0
    │   │   │   ├── atomic-waker v1.1.0
    │   │   │   ├── fastrand v1.9.0
    │   │   │   └── futures-lite v1.12.0 (*)
    │   │   └── futures-lite v1.12.0 (*)
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   ├── async-io v1.12.0
    │   │   ├── async-lock v2.7.0 (*)
    │   │   ├── concurrent-queue v2.1.0 (*)
    │   │   ├── futures-lite v1.12.0 (*)
    │   │   ├── libc v0.2.139
    │   │   ├── log v0.4.17 (*)
    │   │   ├── parking v2.0.0
    │   │   ├── polling v2.5.2
    │   │   │   ├── cfg-if v1.0.0
    │   │   │   ├── libc v0.2.139
    │   │   │   └── log v0.4.17 (*)
    │   │   │   [build-dependencies]
    │   │   │   └── autocfg v1.1.0
    │   │   ├── slab v0.4.8 (*)
    │   │   ├── socket2 v0.4.9
    │   │   │   └── libc v0.2.139
    │   │   └── waker-fn v1.1.0
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   ├── async-lock v2.7.0 (*)
    │   ├── async-net v1.7.0
    │   │   ├── async-io v1.12.0 (*)
    │   │   ├── blocking v1.3.0 (*)
    │   │   └── futures-lite v1.12.0 (*)
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   ├── async-process v1.6.0
    │   │   ├── async-io v1.12.0 (*)
    │   │   ├── async-lock v2.7.0 (*)
    │   │   ├── cfg-if v1.0.0
    │   │   ├── event-listener v2.5.3
    │   │   ├── futures-lite v1.12.0 (*)
    │   │   ├── libc v0.2.139
    │   │   └── signal-hook v0.3.15
    │   │       ├── libc v0.2.139
    │   │       └── signal-hook-registry v1.4.1
    │   │           └── libc v0.2.139
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   ├── blocking v1.3.0 (*)
    │   └── futures-lite v1.12.0 (*)
    ├── subtle v2.5.0
    └── verhoeff v1.0.0
[build-dependencies]
├── anyhow v1.0.69
└── embuild v0.31.0 (*)
```
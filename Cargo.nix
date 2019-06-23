# Generated by carnix 0.9.8: carnix generate-nix --src ../. --standalone
with import <nixpkgs> {};
with buildRustCrateHelpers;
let inherit (lib.lists) fold;
    inherit (lib.attrsets) recursiveUpdate;
  cratesIO = (callPackage ./crates-io.nix { });
in
rec {
  crates = cratesIO // rec {
# mctui-0.1.0

    crates.mctui."0.1.0" = deps: { features?(features_.mctui."0.1.0" deps {}) }: buildRustCrate {
      crateName = "mctui";
      version = "0.1.0";
      authors = [ "noituri" ];
      edition = "2018";
      src = exclude [ ".git" "target" ] ./.;
      dependencies = mapFeatures features ([
        (cratesIO.crates."crossbeam_channel"."${deps."mctui"."0.1.0"."crossbeam_channel"}" deps)
        (cratesIO.crates."failure"."${deps."mctui"."0.1.0"."failure"}" deps)
        (cratesIO.crates."lazy_static"."${deps."mctui"."0.1.0"."lazy_static"}" deps)
        (cratesIO.crates."reqwest"."${deps."mctui"."0.1.0"."reqwest"}" deps)
        (cratesIO.crates."serde"."${deps."mctui"."0.1.0"."serde"}" deps)
        (cratesIO.crates."serde_json"."${deps."mctui"."0.1.0"."serde_json"}" deps)
        (cratesIO.crates."sha_1"."${deps."mctui"."0.1.0"."sha_1"}" deps)
        (cratesIO.crates."termion"."${deps."mctui"."0.1.0"."termion"}" deps)
        (cratesIO.crates."tui"."${deps."mctui"."0.1.0"."tui"}" deps)
        (cratesIO.crates."uuid"."${deps."mctui"."0.1.0"."uuid"}" deps)
      ]);
    };
    features_.mctui."0.1.0" = deps: f: updateFeatures f (rec {
      crossbeam_channel."${deps.mctui."0.1.0".crossbeam_channel}".default = true;
      failure."${deps.mctui."0.1.0".failure}".default = true;
      lazy_static."${deps.mctui."0.1.0".lazy_static}".default = true;
      mctui."0.1.0".default = (f.mctui."0.1.0".default or true);
      reqwest."${deps.mctui."0.1.0".reqwest}".default = true;
      serde."${deps.mctui."0.1.0".serde}".default = true;
      serde_json."${deps.mctui."0.1.0".serde_json}".default = true;
      sha_1."${deps.mctui."0.1.0".sha_1}".default = true;
      termion."${deps.mctui."0.1.0".termion}".default = true;
      tui."${deps.mctui."0.1.0".tui}".default = true;
      uuid."${deps.mctui."0.1.0".uuid}".default = true;
    }) [
      (cratesIO.features_.crossbeam_channel."${deps."mctui"."0.1.0"."crossbeam_channel"}" deps)
      (cratesIO.features_.failure."${deps."mctui"."0.1.0"."failure"}" deps)
      (cratesIO.features_.lazy_static."${deps."mctui"."0.1.0"."lazy_static"}" deps)
      (cratesIO.features_.reqwest."${deps."mctui"."0.1.0"."reqwest"}" deps)
      (cratesIO.features_.serde."${deps."mctui"."0.1.0"."serde"}" deps)
      (cratesIO.features_.serde_json."${deps."mctui"."0.1.0"."serde_json"}" deps)
      (cratesIO.features_.sha_1."${deps."mctui"."0.1.0"."sha_1"}" deps)
      (cratesIO.features_.termion."${deps."mctui"."0.1.0"."termion"}" deps)
      (cratesIO.features_.tui."${deps."mctui"."0.1.0"."tui"}" deps)
      (cratesIO.features_.uuid."${deps."mctui"."0.1.0"."uuid"}" deps)
    ];


# end

  };

  mctui = crates.crates.mctui."0.1.0" deps;
  __all = [ (mctui {}) ];
  deps.adler32."1.0.3" = {};
  deps.aho_corasick."0.7.3" = {
    memchr = "2.2.0";
  };
  deps.arrayvec."0.4.10" = {
    nodrop = "0.1.13";
  };
  deps.autocfg."0.1.4" = {};
  deps.backtrace."0.3.31" = {
    backtrace_sys = "0.1.28";
    cfg_if = "0.1.9";
    libc = "0.2.58";
    rustc_demangle = "0.1.15";
    autocfg = "0.1.4";
  };
  deps.backtrace_sys."0.1.28" = {
    libc = "0.2.58";
    cc = "1.0.37";
  };
  deps.base64."0.10.1" = {
    byteorder = "1.3.2";
  };
  deps.bitflags."1.1.0" = {};
  deps.block_buffer."0.7.3" = {
    block_padding = "0.1.4";
    byte_tools = "0.3.1";
    byteorder = "1.3.2";
    generic_array = "0.12.0";
  };
  deps.block_padding."0.1.4" = {
    byte_tools = "0.3.1";
  };
  deps.build_const."0.2.1" = {};
  deps.byte_tools."0.3.1" = {};
  deps.byteorder."1.3.2" = {};
  deps.bytes."0.4.12" = {
    byteorder = "1.3.2";
    either = "1.5.2";
    iovec = "0.1.2";
  };
  deps.cassowary."0.3.0" = {};
  deps.cc."1.0.37" = {};
  deps.cfg_if."0.1.9" = {};
  deps.cloudabi."0.0.3" = {
    bitflags = "1.1.0";
  };
  deps.cookie."0.12.0" = {
    time = "0.1.42";
    url = "1.7.2";
  };
  deps.cookie_store."0.7.0" = {
    cookie = "0.12.0";
    failure = "0.1.5";
    idna = "0.1.5";
    log = "0.4.6";
    publicsuffix = "1.5.2";
    serde = "1.0.93";
    serde_json = "1.0.39";
    time = "0.1.42";
    try_from = "0.3.2";
    url = "1.7.2";
  };
  deps.core_foundation."0.6.4" = {
    core_foundation_sys = "0.6.2";
    libc = "0.2.58";
  };
  deps.core_foundation_sys."0.6.2" = {};
  deps.crc."1.8.1" = {
    build_const = "0.2.1";
  };
  deps.crc32fast."1.2.0" = {
    cfg_if = "0.1.9";
  };
  deps.crossbeam_channel."0.3.8" = {
    crossbeam_utils = "0.6.5";
    smallvec = "0.6.10";
  };
  deps.crossbeam_deque."0.7.1" = {
    crossbeam_epoch = "0.7.1";
    crossbeam_utils = "0.6.5";
  };
  deps.crossbeam_epoch."0.7.1" = {
    arrayvec = "0.4.10";
    cfg_if = "0.1.9";
    crossbeam_utils = "0.6.5";
    lazy_static = "1.3.0";
    memoffset = "0.2.1";
    scopeguard = "0.3.3";
  };
  deps.crossbeam_queue."0.1.2" = {
    crossbeam_utils = "0.6.5";
  };
  deps.crossbeam_utils."0.6.5" = {
    cfg_if = "0.1.9";
    lazy_static = "1.3.0";
  };
  deps.digest."0.8.0" = {
    generic_array = "0.12.0";
  };
  deps.dtoa."0.4.4" = {};
  deps.either."1.5.2" = {};
  deps.encoding_rs."0.8.17" = {
    cfg_if = "0.1.9";
  };
  deps.error_chain."0.12.1" = {
    backtrace = "0.3.31";
    version_check = "0.1.5";
  };
  deps.failure."0.1.5" = {
    backtrace = "0.3.31";
    failure_derive = "0.1.5";
  };
  deps.failure_derive."0.1.5" = {
    proc_macro2 = "0.4.30";
    quote = "0.6.12";
    syn = "0.15.37";
    synstructure = "0.10.2";
  };
  deps.fake_simd."0.1.2" = {};
  deps.flate2."1.0.9" = {
    crc32fast = "1.2.0";
    libc = "0.2.58";
    miniz_oxide_c_api = "0.2.1";
  };
  deps.fnv."1.0.6" = {};
  deps.foreign_types."0.3.2" = {
    foreign_types_shared = "0.1.1";
  };
  deps.foreign_types_shared."0.1.1" = {};
  deps.fuchsia_cprng."0.1.1" = {};
  deps.fuchsia_zircon."0.3.3" = {
    bitflags = "1.1.0";
    fuchsia_zircon_sys = "0.3.3";
  };
  deps.fuchsia_zircon_sys."0.3.3" = {};
  deps.futures."0.1.27" = {};
  deps.futures_cpupool."0.1.8" = {
    futures = "0.1.27";
    num_cpus = "1.10.1";
  };
  deps.generic_array."0.12.0" = {
    typenum = "1.10.0";
  };
  deps.h2."0.1.24" = {
    byteorder = "1.3.2";
    bytes = "0.4.12";
    fnv = "1.0.6";
    futures = "0.1.27";
    http = "0.1.17";
    indexmap = "1.0.2";
    log = "0.4.6";
    slab = "0.4.2";
    string = "0.2.0";
    tokio_io = "0.1.12";
  };
  deps.http."0.1.17" = {
    bytes = "0.4.12";
    fnv = "1.0.6";
    itoa = "0.4.4";
  };
  deps.http_body."0.1.0" = {
    bytes = "0.4.12";
    futures = "0.1.27";
    http = "0.1.17";
    tokio_buf = "0.1.1";
  };
  deps.httparse."1.3.3" = {};
  deps.hyper."0.12.30" = {
    bytes = "0.4.12";
    futures = "0.1.27";
    futures_cpupool = "0.1.8";
    h2 = "0.1.24";
    http = "0.1.17";
    http_body = "0.1.0";
    httparse = "1.3.3";
    iovec = "0.1.2";
    itoa = "0.4.4";
    log = "0.4.6";
    net2 = "0.2.33";
    time = "0.1.42";
    tokio = "0.1.21";
    tokio_buf = "0.1.1";
    tokio_executor = "0.1.7";
    tokio_io = "0.1.12";
    tokio_reactor = "0.1.9";
    tokio_tcp = "0.1.3";
    tokio_threadpool = "0.1.14";
    tokio_timer = "0.2.11";
    want = "0.0.6";
    rustc_version = "0.2.3";
  };
  deps.hyper_tls."0.3.2" = {
    bytes = "0.4.12";
    futures = "0.1.27";
    hyper = "0.12.30";
    native_tls = "0.2.3";
    tokio_io = "0.1.12";
  };
  deps.idna."0.1.5" = {
    matches = "0.1.8";
    unicode_bidi = "0.3.4";
    unicode_normalization = "0.1.8";
  };
  deps.indexmap."1.0.2" = {};
  deps.iovec."0.1.2" = {
    libc = "0.2.58";
    winapi = "0.2.8";
  };
  deps.itertools."0.8.0" = {
    either = "1.5.2";
  };
  deps.itoa."0.4.4" = {};
  deps.kernel32_sys."0.2.2" = {
    winapi = "0.2.8";
    winapi_build = "0.1.1";
  };
  deps.lazy_static."1.3.0" = {};
  deps.libc."0.2.58" = {};
  deps.lock_api."0.1.5" = {
    owning_ref = "0.4.0";
    scopeguard = "0.3.3";
  };
  deps.log."0.4.6" = {
    cfg_if = "0.1.9";
  };
  deps.matches."0.1.8" = {};
  deps.mctui."0.1.0" = {
    crossbeam_channel = "0.3.8";
    failure = "0.1.5";
    lazy_static = "1.3.0";
    reqwest = "0.9.18";
    serde = "1.0.93";
    serde_json = "1.0.39";
    sha_1 = "0.8.1";
    termion = "1.5.3";
    tui = "0.6.1";
    uuid = "0.7.4";
  };
  deps.memchr."2.2.0" = {};
  deps.memoffset."0.2.1" = {};
  deps.mime."0.3.13" = {
    unicase = "2.4.0";
  };
  deps.mime_guess."2.0.0-alpha.6" = {
    mime = "0.3.13";
    phf = "0.7.24";
    unicase = "1.4.2";
    phf_codegen = "0.7.24";
  };
  deps.miniz_oxide."0.2.1" = {
    adler32 = "1.0.3";
  };
  deps.miniz_oxide_c_api."0.2.1" = {
    crc = "1.8.1";
    libc = "0.2.58";
    miniz_oxide = "0.2.1";
    cc = "1.0.37";
  };
  deps.mio."0.6.19" = {
    iovec = "0.1.2";
    log = "0.4.6";
    net2 = "0.2.33";
    slab = "0.4.2";
    fuchsia_zircon = "0.3.3";
    fuchsia_zircon_sys = "0.3.3";
    libc = "0.2.58";
    kernel32_sys = "0.2.2";
    miow = "0.2.1";
    winapi = "0.2.8";
  };
  deps.miow."0.2.1" = {
    kernel32_sys = "0.2.2";
    net2 = "0.2.33";
    winapi = "0.2.8";
    ws2_32_sys = "0.2.1";
  };
  deps.native_tls."0.2.3" = {
    lazy_static = "1.3.0";
    libc = "0.2.58";
    security_framework = "0.3.1";
    security_framework_sys = "0.3.1";
    tempfile = "3.0.8";
    log = "0.4.6";
    openssl = "0.10.23";
    openssl_probe = "0.1.2";
    openssl_sys = "0.9.47";
    schannel = "0.1.15";
  };
  deps.net2."0.2.33" = {
    cfg_if = "0.1.9";
    libc = "0.2.58";
    winapi = "0.3.7";
  };
  deps.nodrop."0.1.13" = {};
  deps.num_cpus."1.10.1" = {
    libc = "0.2.58";
  };
  deps.numtoa."0.1.0" = {};
  deps.opaque_debug."0.2.2" = {};
  deps.openssl."0.10.23" = {
    bitflags = "1.1.0";
    cfg_if = "0.1.9";
    foreign_types = "0.3.2";
    lazy_static = "1.3.0";
    libc = "0.2.58";
    openssl_sys = "0.9.47";
  };
  deps.openssl_probe."0.1.2" = {};
  deps.openssl_sys."0.9.47" = {
    libc = "0.2.58";
    autocfg = "0.1.4";
    cc = "1.0.37";
    pkg_config = "0.3.14";
  };
  deps.owning_ref."0.4.0" = {
    stable_deref_trait = "1.1.1";
  };
  deps.parking_lot."0.7.1" = {
    lock_api = "0.1.5";
    parking_lot_core = "0.4.0";
  };
  deps.parking_lot_core."0.4.0" = {
    rand = "0.6.5";
    smallvec = "0.6.10";
    rustc_version = "0.2.3";
    libc = "0.2.58";
    winapi = "0.3.7";
  };
  deps.percent_encoding."1.0.1" = {};
  deps.phf."0.7.24" = {
    phf_shared = "0.7.24";
  };
  deps.phf_codegen."0.7.24" = {
    phf_generator = "0.7.24";
    phf_shared = "0.7.24";
  };
  deps.phf_generator."0.7.24" = {
    phf_shared = "0.7.24";
    rand = "0.6.5";
  };
  deps.phf_shared."0.7.24" = {
    siphasher = "0.2.3";
    unicase = "1.4.2";
  };
  deps.pkg_config."0.3.14" = {};
  deps.proc_macro2."0.4.30" = {
    unicode_xid = "0.1.0";
  };
  deps.publicsuffix."1.5.2" = {
    error_chain = "0.12.1";
    idna = "0.1.5";
    lazy_static = "1.3.0";
    regex = "1.1.7";
    url = "1.7.2";
  };
  deps.quote."0.6.12" = {
    proc_macro2 = "0.4.30";
  };
  deps.rand."0.6.5" = {
    rand_chacha = "0.1.1";
    rand_core = "0.4.0";
    rand_hc = "0.1.0";
    rand_isaac = "0.1.1";
    rand_jitter = "0.1.4";
    rand_os = "0.1.3";
    rand_pcg = "0.1.2";
    rand_xorshift = "0.1.1";
    autocfg = "0.1.4";
    libc = "0.2.58";
    winapi = "0.3.7";
  };
  deps.rand_chacha."0.1.1" = {
    rand_core = "0.3.1";
    autocfg = "0.1.4";
  };
  deps.rand_core."0.3.1" = {
    rand_core = "0.4.0";
  };
  deps.rand_core."0.4.0" = {};
  deps.rand_hc."0.1.0" = {
    rand_core = "0.3.1";
  };
  deps.rand_isaac."0.1.1" = {
    rand_core = "0.3.1";
  };
  deps.rand_jitter."0.1.4" = {
    rand_core = "0.4.0";
    libc = "0.2.58";
    winapi = "0.3.7";
  };
  deps.rand_os."0.1.3" = {
    rand_core = "0.4.0";
    rdrand = "0.4.0";
    cloudabi = "0.0.3";
    fuchsia_cprng = "0.1.1";
    libc = "0.2.58";
    winapi = "0.3.7";
  };
  deps.rand_pcg."0.1.2" = {
    rand_core = "0.4.0";
    autocfg = "0.1.4";
  };
  deps.rand_xorshift."0.1.1" = {
    rand_core = "0.3.1";
  };
  deps.rdrand."0.4.0" = {
    rand_core = "0.3.1";
  };
  deps.redox_syscall."0.1.54" = {};
  deps.redox_termios."0.1.1" = {
    redox_syscall = "0.1.54";
  };
  deps.regex."1.1.7" = {
    aho_corasick = "0.7.3";
    memchr = "2.2.0";
    regex_syntax = "0.6.7";
    thread_local = "0.3.6";
    utf8_ranges = "1.0.3";
  };
  deps.regex_syntax."0.6.7" = {
    ucd_util = "0.1.3";
  };
  deps.remove_dir_all."0.5.2" = {
    winapi = "0.3.7";
  };
  deps.reqwest."0.9.18" = {
    base64 = "0.10.1";
    bytes = "0.4.12";
    cookie = "0.12.0";
    cookie_store = "0.7.0";
    encoding_rs = "0.8.17";
    flate2 = "1.0.9";
    futures = "0.1.27";
    http = "0.1.17";
    hyper = "0.12.30";
    hyper_tls = "0.3.2";
    log = "0.4.6";
    mime = "0.3.13";
    mime_guess = "2.0.0-alpha.6";
    native_tls = "0.2.3";
    serde = "1.0.93";
    serde_json = "1.0.39";
    serde_urlencoded = "0.5.5";
    time = "0.1.42";
    tokio = "0.1.21";
    tokio_executor = "0.1.7";
    tokio_io = "0.1.12";
    tokio_threadpool = "0.1.14";
    tokio_timer = "0.2.11";
    url = "1.7.2";
    uuid = "0.7.4";
  };
  deps.rustc_demangle."0.1.15" = {};
  deps.rustc_version."0.2.3" = {
    semver = "0.9.0";
  };
  deps.ryu."0.2.8" = {};
  deps.schannel."0.1.15" = {
    lazy_static = "1.3.0";
    winapi = "0.3.7";
  };
  deps.scopeguard."0.3.3" = {};
  deps.security_framework."0.3.1" = {
    core_foundation = "0.6.4";
    core_foundation_sys = "0.6.2";
    libc = "0.2.58";
    security_framework_sys = "0.3.1";
  };
  deps.security_framework_sys."0.3.1" = {
    core_foundation_sys = "0.6.2";
  };
  deps.semver."0.9.0" = {
    semver_parser = "0.7.0";
  };
  deps.semver_parser."0.7.0" = {};
  deps.serde."1.0.93" = {
    serde_derive = "1.0.93";
  };
  deps.serde_derive."1.0.93" = {
    proc_macro2 = "0.4.30";
    quote = "0.6.12";
    syn = "0.15.37";
  };
  deps.serde_json."1.0.39" = {
    itoa = "0.4.4";
    ryu = "0.2.8";
    serde = "1.0.93";
  };
  deps.serde_urlencoded."0.5.5" = {
    dtoa = "0.4.4";
    itoa = "0.4.4";
    serde = "1.0.93";
    url = "1.7.2";
  };
  deps.sha_1."0.8.1" = {
    block_buffer = "0.7.3";
    digest = "0.8.0";
    fake_simd = "0.1.2";
    opaque_debug = "0.2.2";
  };
  deps.siphasher."0.2.3" = {};
  deps.slab."0.4.2" = {};
  deps.smallvec."0.6.10" = {};
  deps.stable_deref_trait."1.1.1" = {};
  deps.string."0.2.0" = {
    bytes = "0.4.12";
  };
  deps.syn."0.15.37" = {
    proc_macro2 = "0.4.30";
    quote = "0.6.12";
    unicode_xid = "0.1.0";
  };
  deps.synstructure."0.10.2" = {
    proc_macro2 = "0.4.30";
    quote = "0.6.12";
    syn = "0.15.37";
    unicode_xid = "0.1.0";
  };
  deps.tempfile."3.0.8" = {
    cfg_if = "0.1.9";
    rand = "0.6.5";
    remove_dir_all = "0.5.2";
    redox_syscall = "0.1.54";
    libc = "0.2.58";
    winapi = "0.3.7";
  };
  deps.termion."1.5.3" = {
    numtoa = "0.1.0";
    libc = "0.2.58";
    redox_syscall = "0.1.54";
    redox_termios = "0.1.1";
  };
  deps.thread_local."0.3.6" = {
    lazy_static = "1.3.0";
  };
  deps.time."0.1.42" = {
    libc = "0.2.58";
    redox_syscall = "0.1.54";
    winapi = "0.3.7";
  };
  deps.tokio."0.1.21" = {
    bytes = "0.4.12";
    futures = "0.1.27";
    mio = "0.6.19";
    num_cpus = "1.10.1";
    tokio_current_thread = "0.1.6";
    tokio_executor = "0.1.7";
    tokio_io = "0.1.12";
    tokio_reactor = "0.1.9";
    tokio_tcp = "0.1.3";
    tokio_threadpool = "0.1.14";
    tokio_timer = "0.2.11";
    tokio_trace_core = "0.2.0";
  };
  deps.tokio_buf."0.1.1" = {
    bytes = "0.4.12";
    either = "1.5.2";
    futures = "0.1.27";
  };
  deps.tokio_current_thread."0.1.6" = {
    futures = "0.1.27";
    tokio_executor = "0.1.7";
  };
  deps.tokio_executor."0.1.7" = {
    crossbeam_utils = "0.6.5";
    futures = "0.1.27";
  };
  deps.tokio_io."0.1.12" = {
    bytes = "0.4.12";
    futures = "0.1.27";
    log = "0.4.6";
  };
  deps.tokio_reactor."0.1.9" = {
    crossbeam_utils = "0.6.5";
    futures = "0.1.27";
    lazy_static = "1.3.0";
    log = "0.4.6";
    mio = "0.6.19";
    num_cpus = "1.10.1";
    parking_lot = "0.7.1";
    slab = "0.4.2";
    tokio_executor = "0.1.7";
    tokio_io = "0.1.12";
    tokio_sync = "0.1.6";
  };
  deps.tokio_sync."0.1.6" = {
    fnv = "1.0.6";
    futures = "0.1.27";
  };
  deps.tokio_tcp."0.1.3" = {
    bytes = "0.4.12";
    futures = "0.1.27";
    iovec = "0.1.2";
    mio = "0.6.19";
    tokio_io = "0.1.12";
    tokio_reactor = "0.1.9";
  };
  deps.tokio_threadpool."0.1.14" = {
    crossbeam_deque = "0.7.1";
    crossbeam_queue = "0.1.2";
    crossbeam_utils = "0.6.5";
    futures = "0.1.27";
    log = "0.4.6";
    num_cpus = "1.10.1";
    rand = "0.6.5";
    slab = "0.4.2";
    tokio_executor = "0.1.7";
  };
  deps.tokio_timer."0.2.11" = {
    crossbeam_utils = "0.6.5";
    futures = "0.1.27";
    slab = "0.4.2";
    tokio_executor = "0.1.7";
  };
  deps.tokio_trace_core."0.2.0" = {
    lazy_static = "1.3.0";
  };
  deps.try_lock."0.2.2" = {};
  deps.try_from."0.3.2" = {
    cfg_if = "0.1.9";
  };
  deps.tui."0.6.1" = {
    bitflags = "1.1.0";
    cassowary = "0.3.0";
    either = "1.5.2";
    itertools = "0.8.0";
    log = "0.4.6";
    termion = "1.5.3";
    unicode_segmentation = "1.3.0";
    unicode_width = "0.1.5";
  };
  deps.typenum."1.10.0" = {};
  deps.ucd_util."0.1.3" = {};
  deps.unicase."1.4.2" = {
    version_check = "0.1.5";
  };
  deps.unicase."2.4.0" = {
    version_check = "0.1.5";
  };
  deps.unicode_bidi."0.3.4" = {
    matches = "0.1.8";
  };
  deps.unicode_normalization."0.1.8" = {
    smallvec = "0.6.10";
  };
  deps.unicode_segmentation."1.3.0" = {};
  deps.unicode_width."0.1.5" = {};
  deps.unicode_xid."0.1.0" = {};
  deps.url."1.7.2" = {
    idna = "0.1.5";
    matches = "0.1.8";
    percent_encoding = "1.0.1";
  };
  deps.utf8_ranges."1.0.3" = {};
  deps.uuid."0.7.4" = {
    rand = "0.6.5";
  };
  deps.vcpkg."0.2.6" = {};
  deps.version_check."0.1.5" = {};
  deps.want."0.0.6" = {
    futures = "0.1.27";
    log = "0.4.6";
    try_lock = "0.2.2";
  };
  deps.winapi."0.2.8" = {};
  deps.winapi."0.3.7" = {
    winapi_i686_pc_windows_gnu = "0.4.0";
    winapi_x86_64_pc_windows_gnu = "0.4.0";
  };
  deps.winapi_build."0.1.1" = {};
  deps.winapi_i686_pc_windows_gnu."0.4.0" = {};
  deps.winapi_x86_64_pc_windows_gnu."0.4.0" = {};
  deps.ws2_32_sys."0.2.1" = {
    winapi = "0.2.8";
    winapi_build = "0.1.1";
  };
}

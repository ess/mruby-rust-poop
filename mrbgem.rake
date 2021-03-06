require 'open3'
require 'fileutils'
MRuby::Gem::Specification.new('mruby-rust-poop') do |spec|
  spec.license = 'MIT'
  spec.authors = 'ess'
  spec.version = '0.0.1'
  spec.description = 'Rust regexp for MRuby'

  spec.add_dependency 'mruby-mtest',     mgem: 'mruby-mtest'
  spec.add_dependency 'mferuby-runtime', github: 'hone/mferuby', branch: 'wip'

  def run_command env, command
    STDOUT.sync = true
    puts "build: [exec] #{command}"
    Open3.popen2e(env, command) do |stdin, stdout, thread|
      print stdout.read
      fail "#{command} failed" if thread.value != 0
    end
  end

  FileUtils.mkdir_p build_dir

  Info      = Struct.new(:cargo_target, :linker_libraries)
  arch_info = {
    "x86_64-pc-linux-gnu"   => Info.new("x86_64-unknown-linux-gnu", %w(pthread dl)),
    "i686-pc-linux-gnu"     => Info.new("i686-unknown-linux-gnu", %w(pthread dl)),
    "x86_64-apple-darwin14" => Info.new("x86_64-apple-darwin", []),
    "i386-apple-darwin14"   => Info.new("i686-apple-darwin", []),
    "x86_64-w64-mingw32"    => Info.new("x86_64-pc-windows-gnu", %w(ws2_32 userenv shell32 advapi32)),
    "i686-w64-mingw32"      => Info.new("i686-pc-windows-gnu", %w(ws2_32 userenv shell32 advapi32))
  }
  e = {
    "CARGO_TARGET_DIR" => build_dir
  }
  rust_lib_name = "mruby_rust_poop"
  cargo_command = "cargo build"

  spec.linker.libraries << rust_lib_name

  if build.kind_of?(MRuby::CrossBuild) && build.host_target
    target = arch_info[build.host_target].cargo_target
    cargo_command << " --target=#{target}"
    spec.linker.library_paths << "#{build_dir}/#{target}/debug"
    # force mruby build to generate gem init
    if target.include?("windows")
      spec.objs << libfile("#{spec.build_dir}/#{target}/debug/#{rust_lib_name}")
    else
      spec.objs << libfile("#{spec.build_dir}/#{target}/debug/lib#{rust_lib_name}")
    end
    arch_info[build.host_target].linker_libraries.each do |lib|
      spec.linker.libraries << lib
    end
    spec.linker.flags_after_libraries << "-Wl,-no_compact_unwind" if build.host_target == "i386-apple-darwin14"
  else
    # host / 64-bit linux build
    arch_info["x86_64-pc-linux-gnu"].linker_libraries.each do |lib|
      spec.linker.libraries << lib
    end
    spec.linker.library_paths << "#{build_dir}/debug"
    # force mruby build to generate gem init
    spec.objs << libfile("#{spec.build_dir}/debug/lib#{rust_lib_name}")
  end

  rust_dir = File.join(File.dirname(File.realpath(__FILE__)), "rust")
  Dir.chdir(rust_dir) do
    run_command e, cargo_command
  end
end

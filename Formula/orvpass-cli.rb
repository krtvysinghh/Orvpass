class OrvpassCli < Formula
  desc "Ultra-fast, zero-knowledge terminal password & secrets manager"
  homepage "https://github.com/krtvysinghh/Orvpass"
  version "5.0.0"
  license "Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/krtvysinghh/Orvpass/releases/download/v5.0.0/orvpass-v5.0.0-macos-arm64.tar.gz"
      sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    else
      url "https://github.com/krtvysinghh/Orvpass/releases/download/v5.0.0/orvpass-v5.0.0-macos-x86_64.tar.gz"
      sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    end
  end

  on_linux do
    url "https://github.com/krtvysinghh/Orvpass/releases/download/v5.0.0/orvpass-v5.0.0-linux-x86_64.tar.gz"
    sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  end

  def install
    bin.install "orvpass"
    
    # Generate shell completions
    generate_completions_from_executable(bin/"orvpass", "completions")
  end

  test do
    system "#{bin}/orvpass", "--version"
    system "#{bin}/orvpass", "doctor"
  end
end

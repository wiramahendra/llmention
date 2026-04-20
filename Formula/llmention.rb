class Llmention < Formula
  desc "Local-first GEO agent: track, generate, and optimize your brand's visibility in LLMs"
  homepage "https://github.com/wiramahendra/llMention"
  version "0.3.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/wiramahendra/llMention/releases/download/v#{version}/llmention-macos-aarch64.tar.gz"
      sha256 "PLACEHOLDER_AARCH64_SHA256"
    end
    on_intel do
      url "https://github.com/wiramahendra/llMention/releases/download/v#{version}/llmention-macos-x86_64.tar.gz"
      sha256 "PLACEHOLDER_X86_64_SHA256"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/wiramahendra/llMention/releases/download/v#{version}/llmention-linux-x86_64.tar.gz"
      sha256 "PLACEHOLDER_LINUX_SHA256"
    end
  end

  def install
    bin.install "llmention"
  end

  def caveats
    <<~EOS
      To get started, run:
        llmention init

      This will walk you through setting up your first LLM provider and domain to track.

      Documentation:
        llmention docs
        llmention quickstart
    EOS
  end

  test do
    assert_match "llmention", shell_output("#{bin}/llmention --version")
  end
end

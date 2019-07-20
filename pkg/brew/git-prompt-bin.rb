class GitPromptBin < Formula
  version '0.2.0'
  desc "Print the current git status in your prompt"
  homepage "https://github.com/aignas/git-prompt-rs"
  depends_on "openssl@1.1"

  if OS.mac?
      url "https://github.com/aignas/git-prompt-rs/releases/download/#{version}/git-prompt-rs-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "e6e4bfc4f606a73e58cf4eb17c41bc9c6946ee7a423e3da44f62a1c770d2b662"
  elsif OS.linux?
      url "https://github.com/aignas/git-prompt-rs/releases/download/#{version}/git-prompt-rs-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "ea479b54e79983260a54c224b7a15f52c39513f9996f049fecc1dbbae39e9e34"
  end

  #TODO @aignas (2019-06-17): this should be enabled once I have the pkg in main repos
  #conflicts_with "git-prompt"

  def install
    bin.install "git-prompt"
    man1.install "doc/git-prompt.1"

    bash_completion.install "complete/git-prompt.bash"
    fish_completion.install "complete/git-prompt.fish"
    zsh_completion.install "complete/_git-prompt"
  end
end

class GitPromptBin < Formula
  version '0.2.0'
  desc "Print the current git status in your prompt"
  homepage "https://github.com/aignas/git-prompt-rs"

  if OS.mac?
      url "https://github.com/aignas/git-prompt-rs/releases/download/#{version}/git-prompt-rs-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "32754b4173ac87a7bfffd436d601a49362676eb1841ab33440f2f49c002c8967"
  elsif OS.linux?
      url "https://github.com/aignas/git-prompt-rs/releases/download/#{version}/git-prompt-rs-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "c76080aa807a339b44139885d77d15ad60ab8cdd2c2fdaf345d0985625bc0f97"
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

cask "orvpass" do
  version "4.0.8"
  sha256 :no_check

  url "https://github.com/krtvysinghh/Orvpass/releases/download/v#{version}/Orvpass_#{version}_aarch64.dmg"
  name "Orvpass"
  desc "Lightning-fast, highly secure, natively designed password manager"
  homepage "https://github.com/krtvysinghh/Orvpass"

  app "Orvpass.app"

  zap trash: [
    "~/Library/Application Support/Orvpass",
    "~/Library/Caches/Orvpass",
    "~/Library/Preferences/com.krtvysingh.orvpass.plist",
    "~/Library/Saved Application State/com.krtvysingh.orvpass.savedState",
  ]
end

cask "orvpass" do
  version "4.0.1"
  sha256 :no_check # In production, replace with actual SHA256 of the DMG

  url "https://github.com/krtvysinghh/Orvpass/releases/download/v#{version}/Orvpass_#{version}_aarch64.dmg"
  name "Orvpass"
  desc "Lightning-fast, highly secure, natively designed password manager"
  homepage "https://github.com/krtvysinghh/Orvpass"

  app "Orvpass.app"

  zap trash: [
    "~/Library/Application Support/Orvpass",
    "~/Library/Caches/Orvpass",
    "~/Library/Preferences/com.krtvysingh.app.plist",
    "~/Library/Saved Application State/com.krtvysingh.app.savedState",
  ]
end

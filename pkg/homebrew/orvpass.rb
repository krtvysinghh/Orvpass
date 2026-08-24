cask "orvpass" do
  version "4.2.0"
  sha256 "1f0f5b039265f12b84e1d0d13388ea9b8bb155941fb7c18035406f08610ad80d"

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

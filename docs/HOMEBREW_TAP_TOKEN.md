# Homebrew Tap Token Setup

This token allows the release automation to update the `juanjecilla/homebrew-tap` repository.

## Create the token

1. Go to GitHub Settings -> Developer settings -> Personal access tokens -> Fine-grained tokens, then select "Generate new token".
2. Set Resource owner to your account.
3. Set Repository access to "Only select repositories" and choose `homebrew-tap`.
4. Set Permissions to `Contents: Read and write`.
5. Choose an expiration, generate the token, and copy it.

## Store the secret in this repo

1. Open the `mdtolinkedin-cli` repository on GitHub.
2. Go to Settings -> Secrets and variables -> Actions.
3. Click "New repository secret".
4. Name it `HOMEBREW_TAP_TOKEN` and paste the token value.

## Verify

1. Publish a GitHub Release (the Homebrew workflow runs on `release` -> `published`).
2. The workflow will update the tap if `HOMEBREW_TAP_TOKEN` is present. If it is missing, it will skip safely.

## Notes

If your account uses SSO, authorize the token for the org before use.

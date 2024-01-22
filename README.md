Allows validation of permits created via metamask while still working with permits created following https://github.com/scrtlabs/secret-toolkit/tree/master/packages/permit specs.  Metamask validation taken mostly from https://github.com/kent-3/query-authentication

This adds an optional field to the permit called `signature_type` which signifies how the permit was signed and how it should be validated.

If `signature_type` is not specified, it is assumed that the permit was signed with a SN wallet following SNIP-24 specs
The posisble values of `signature_type` are:
  - `metamask_personal_sign` - the permit has been signed using metamask_personal_sign
  - `legacy` - the permit was signed with a SN wallet following SNIP-24 specs

You're viewing this early as it is entirely untested so far
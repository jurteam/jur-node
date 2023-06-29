# Release management

## Versioning

We use [semantic versioning](https://semver.org).

## How to make a new release from the staging branch

1. Create a new branch for the future version from `staging`: `release/vMAJ.MIN.PATCH` 
2. All new changes to the new release will be merged into the new branch.
3. Open a PR against staging ASAP.
4. When the branch is ready for release create an annotated tag" `git tag -a vMAJ.MIN.PATCH -m "Release vMAJ.MIN.PATCH"`
5. Merge the tag into main: `git checkout main && git merge vMAJ.MIN.PATCH`. Push main and the tag.
6. Done



<a name="v1.2.1"></a>
## [v1.2.1](https://github.com/sdttttt/gcr/compare/v1.2.0.rc.1...v1.2.1) (2021-04-24)

### Chore

* **deps:** bump dialoguer from 0.7.1 to 0.8.0

### Docs

* **version:** 📝 update cargo ver.

### Pull Requests

* Merge pull request [#50](https://github.com/sdttttt/gcr/issues/50) from sdttttt/dependabot/cargo/dialoguer-0.8.0


<a name="v1.2.0.rc.1"></a>
## [v1.2.0.rc.1](https://github.com/sdttttt/gcr/compare/v1.1.0...v1.2.0.rc.1) (2021-04-09)

### Chore

* 📦 update grc config file.
* 📦 delete .idea folder.
* 📦 update tools.
* **chglog:** 📦 add changelog and update package info.
* **deps:** bump console from 0.13.0 to 0.14.1

### Ci

* 🚀 add build cache.
* **workflow:** 🚀 delete unit test on windows actions.
* **workflow:** 🚀 Add unit test step.
* **workflow:** 🚀 update build command.
* **workflow:** 🚀 from native makefile to cargo makefile.

### Feat

* 🎉 optimize command exec output.
* 🎉 update pre output.
* 🎉 update command exec logic.
* 🎉 add pre commit action.

### Fix

* **workflow:** 🐞 linux aciton.

### Refactor

* ✂  add after action.

### Test

* 🧪 update unit test.
* **conf:** 🧪 add unit test.

### Pull Requests

* Merge pull request [#60](https://github.com/sdttttt/gcr/issues/60) from sdttttt/develop
* Merge pull request [#59](https://github.com/sdttttt/gcr/issues/59) from sdttttt/develop
* Merge pull request [#55](https://github.com/sdttttt/gcr/issues/55) from sdttttt/sdttttt/fix-workflow
* Merge pull request [#51](https://github.com/sdttttt/gcr/issues/51) from sdttttt/dependabot/cargo/console-0.14.1


<a name="v1.1.0"></a>
## [v1.1.0](https://github.com/sdttttt/gcr/compare/v1.1.0-beta.1...v1.1.0) (2021-03-08)

### Chore

* **cargo:** 📦 upgrade version.

### Pull Requests

* Merge pull request [#43](https://github.com/sdttttt/gcr/issues/43) from sdttttt/develop
* Merge pull request [#42](https://github.com/sdttttt/gcr/issues/42) from sdttttt/develop


<a name="v1.1.0-beta.1"></a>
## [v1.1.0-beta.1](https://github.com/sdttttt/gcr/compare/v1.0.0...v1.1.0-beta.1) (2021-02-19)

### Chore

* **cargo:** 📦 upgrade version.

### Docs

* 📝 update README.
* **plug:** 📝 Add plug system description.

### Feat

* **plug:** 🎉 prefect push plug action.
* **plug:** 🎉 complete push plugin.
* **plug:** 🎉 complete push plugin. (WIP)
* **plug:** 🎉 complete plugin system.
* **plug:** 🎉 (WIP) Add Plugin System.

### Fix

* **plug:** 🐞 remote name is not valid in push.
* **repo:** 🐞 could not first commit.

### Style

* **plug:** 🎨 add push plugin branch output.
* **push-plug:** 🎨 enh output.


<a name="v1.0.0"></a>
## [v1.0.0](https://github.com/sdttttt/gcr/compare/v1.0.0.beta.1...v1.0.0) (2021-02-04)

### Deps

* **cargo:** update lock file.

### Fix

* **msg:** subject problom.

### Version

*  update cargo.toml


<a name="v1.0.0.beta.1"></a>
## [v1.0.0.beta.1](https://github.com/sdttttt/gcr/compare/v0.9.2a...v1.0.0.beta.1) (2021-02-03)

### Chore

* add cargo-husky

### Ci

* limit thread const at UnitTest.
* fix ci

### Docs

* 📝 Add 1.0.0 new feature document.
* fix style.

### Feat

* **ext:** 🎉 Add overwrite_emoji config feild.

### Fix

* 🐞emoji mode determine.

### Refactor

* ✂ emoji mode.
* (WIP) refor config module.
* **args:** Optimize syntax.
* **ext:** ✂  optimize if type = null expection status.
* **unit:** optimize style.

### Style

* **msg:** 🎨Change input tip.

### Test

* optimize
* optimize test style.
* **unit:** fix ci fail.
* **unit:** optimize env test.

### Pull Requests

* Merge pull request [#41](https://github.com/sdttttt/gcr/issues/41) from sdttttt/develop
* Merge pull request [#40](https://github.com/sdttttt/gcr/issues/40) from sdttttt/develop
* Merge pull request [#39](https://github.com/sdttttt/gcr/issues/39) from sdttttt/develop
* Merge pull request [#38](https://github.com/sdttttt/gcr/issues/38) from sdttttt/develop
* Merge pull request [#37](https://github.com/sdttttt/gcr/issues/37) from sdttttt/sdttttt-patch-1
* Merge pull request [#36](https://github.com/sdttttt/gcr/issues/36) from sdttttt/develop


<a name="v0.9.2a"></a>
## [v0.9.2a](https://github.com/sdttttt/gcr/compare/v0.9.2z...v0.9.2a) (2020-11-30)

### Chore

* makefile.


<a name="v0.9.2z"></a>
## [v0.9.2z](https://github.com/sdttttt/gcr/compare/v0.9.2...v0.9.2z) (2020-11-30)

### Ci

* update release work.


<a name="v0.9.2"></a>
## [v0.9.2](https://github.com/sdttttt/gcr/compare/v0.9.1...v0.9.2) (2020-11-30)

### Chore

* fix toolchain.
* Change toolchain. Add coverage HTML report in makefile.
* **fmt:** Update rustfmt config.
* **makefile:** update.

### Ci

* update workflow grcov call path.
* change toolchain.
* Change coverage tool.
* from stable to nightly on linux.
* only master trigger release.
* del pr release.
* fix syn.
* fix syn.
* **workflow:** change toolchain channel to `stable` from `nigth`.
* **workflow:** fix syn.
* **workflow:** enh cache and dump env.
* **workflow.:** fix syntax.

### Docs

* Update CONTRIBUTORS.

### Feat

* **log:** Add WARN print.
* **repo:** Perfect name specification for commit.

### Refactor

* Delete `Auto` and `Push` Mode.
*  Delete Push MODE.
* **args:** Add designate config handle.
* **args:** to chain mode.

### Test

* perfect test.
* **util:** Add unit tests.

### Version

* **cargo:** upgrade to 0.9.2

### Pull Requests

* Merge pull request [#35](https://github.com/sdttttt/gcr/issues/35) from sdttttt/develop


<a name="v0.9.1"></a>
## [v0.9.1](https://github.com/sdttttt/gcr/compare/v0.9.0-rc.2...v0.9.1) (2020-11-14)

### Chore

* delete .vscode
* **cargo:** Perfect package config.
* **makefile:** Update commit target.

### Ci

* Update workflow.
* Add CI trigger on develop branch.
* delete macos workflow upload codecov.
*  delete [stable, beta] build channel.
* Add MacOS release.
* Change workflow syntax.
* Add build artifact upload.
* Optimizer windows caches.
* Add Windows workflow.
* **windows:** Update workflow name.

### Docs

* Perfect raedme.
* Add TODO file.
* Add comments.

### Feat

* Add read git sign from env.
* **ext:** from .config/grc/grc.toml read GRC config file.

### Fix

* env read problem.
* **ext:** Macro expansion should not be used here.
* **workflows:** delete stable comliper.

### Refactor

* **log:** rename `gcr` to `grc`.

### Style

* rustfmt.

### Version

* **cargo:** Upgrade to 0.9.1

### Pull Requests

* Merge pull request [#32](https://github.com/sdttttt/gcr/issues/32) from sdttttt/develop
* Merge pull request [#34](https://github.com/sdttttt/gcr/issues/34) from sdttttt/fix-config-type-check
* Merge pull request [#33](https://github.com/sdttttt/gcr/issues/33) from sdttttt/from-env-read-git-author
* Merge pull request [#31](https://github.com/sdttttt/gcr/issues/31) from sdttttt/global-config-file
* Merge pull request [#29](https://github.com/sdttttt/gcr/issues/29) from sdttttt/develop
* Merge pull request [#27](https://github.com/sdttttt/gcr/issues/27) from sdttttt/add-macos-workflow
* Merge pull request [#24](https://github.com/sdttttt/gcr/issues/24) from codacy-badger/codacy-badge
* Merge pull request [#21](https://github.com/sdttttt/gcr/issues/21) from sdttttt/develop
* Merge pull request [#20](https://github.com/sdttttt/gcr/issues/20) from sdttttt/add-code-of-conduct-1
* Merge pull request [#22](https://github.com/sdttttt/gcr/issues/22) from sdttttt/workflows


<a name="v0.9.0-rc.2"></a>
## [v0.9.0-rc.2](https://github.com/sdttttt/gcr/compare/v0.9.0-rc1...v0.9.0-rc.2) (2020-11-01)

### Chore

* **cargo:** Upgrade GRC version.
* **deps:** bump dialoguer from 0.6.2 to 0.7.1 ([#17](https://github.com/sdttttt/gcr/issues/17))
* **ver:** Change cargo version.

### Fix

* **args:** Compatible with dialoguer 0.7.0

### Pull Requests

* Merge pull request [#19](https://github.com/sdttttt/gcr/issues/19) from sdttttt/develop


<a name="v0.9.0-rc1"></a>
## [v0.9.0-rc1](https://github.com/sdttttt/gcr/compare/v0.8.2...v0.9.0-rc1) (2020-11-01)

### Chore

* Add makefile.
* **deps:** bump git2 from 0.13.11 to 0.13.12 ([#18](https://github.com/sdttttt/gcr/issues/18))
* **fmt:** Add rustfmt configation. and fmt project.
* **makefile:** update commit target.
* **makefile:** add commit.
* **vscode:** delete setting file.

### Ci

* Update workflow configuration.
* optimizer workflow syntax.

### Docs

* update readme.
* **readme:** Append v0.9.0 new future description.

### Feat

* Complete grc access config file, We were happy.
* **cofiguration:** Complete file read and parse.
* **configuration:** this is great!! read config file passed the unit test!
* **messager:** using mode to chain call.

### Fix

* **ci:** workflow.
* **ci:** workflow.
* **messager:** selection index out.
* **messager:** now selection index is true.

### Refactor

* **messager:** function transfer.
* **messager:** Change constructor function name.
* **messager:** Ask type.
* **messager:** Change ask function position.
* **metadata:** Extract constant.

### Style

* syntax.
* fmt project.
* **fmt:** messager code.
* **messager:** Optimizer syntax.

### Test

* **configuration:** Add deserialize unit test.
* **configuration:** Add readfile unit test.
* **configuration:** Add from function unit test.
* **configuration:** Add from_agreement unit test.
* **configuration:** Extract constant.
* **messager:** Add typs_list function unit test..
* **messager:** Add build function unit test.
* **messager:** Add load_ext_td function unit test.

### Version

* Test 0.9.0-rc1 GRC work.

### Pull Requests

* Merge pull request [#14](https://github.com/sdttttt/gcr/issues/14) from sdttttt/develop


<a name="v0.8.2"></a>
## [v0.8.2](https://github.com/sdttttt/gcr/compare/v0.8.1...v0.8.2) (2020-10-31)

### Chore

* Add rust-toolchain file.
* **cargo:** Upgrade GRC version.
* **deps:** bump console from 0.12.0 to 0.13.0 ([#16](https://github.com/sdttttt/gcr/issues/16))

### Fix

* **ci:** workflow.

### Refactor

* **messager:** Ask type.
* **messager:** Change ask function position.

### Style

* fmt project.

### Pull Requests

* Merge pull request [#15](https://github.com/sdttttt/gcr/issues/15) from sdttttt/refactor-ask-type


<a name="v0.8.1"></a>
## [v0.8.1](https://github.com/sdttttt/gcr/compare/v0.8.0...v0.8.1) (2020-10-13)

### Docs

* Add comments.
* **README:** update docs.

### Feat

* **cargo:** Upgrade Version.

### Pull Requests

* Merge pull request [#11](https://github.com/sdttttt/gcr/issues/11) from sdttttt/add-license-1


<a name="v0.8.0"></a>
## [v0.8.0](https://github.com/sdttttt/gcr/compare/v0.7.1...v0.8.0) (2020-09-30)

### Docs

* complete command help information.
* Upgrade cargo version.

### Feat

* Complete file add function.
* Complete command common resolve.
* Add command frame.

### Fix

* File Add optimizer.
* **args:** Complete all mode.

### Refactor

* **repo:** delete The superfluous part.

### Style

* **util:** Optimizer match writing method.

### Test

* Add more unit test.
* Add more command mode.
* **args:** Add command mode unit test.
* **log:** Add unit test.
* **repo:** Add repo new instance Test.
* **repo:** Add new repo instance Test.
* **util:** perfect string test.
* **utils:** Add current_path function Test.

### Pull Requests

* Merge pull request [#10](https://github.com/sdttttt/gcr/issues/10) from sdttttt/temporary
* Merge pull request [#6](https://github.com/sdttttt/gcr/issues/6) from sdttttt/add-unit-test


<a name="v0.7.1"></a>
## [v0.7.1](https://github.com/sdttttt/gcr/compare/v0.7.0...v0.7.1) (2020-09-26)

### Chore

* **cargo.toml:** upgrade crate version.

### Feat

* Add output color.


<a name="v0.7.0"></a>
## [v0.7.0](https://github.com/sdttttt/gcr/compare/v0.6.3...v0.7.0) (2020-09-26)

### Chore

* Update cargo.toml.

### Feat

* Implement the commit specification.
* Add description ask step.
* **yes:** yes

### Pull Requests

* Merge pull request [#3](https://github.com/sdttttt/gcr/issues/3) from sdttttt/add-issue-pr-linked
* Merge pull request [#2](https://github.com/sdttttt/gcr/issues/2) from sdttttt/add-description-step


<a name="v0.6.3"></a>
## [v0.6.3](https://github.com/sdttttt/gcr/compare/v0.6.2...v0.6.3) (2020-09-25)

### Ci

* **githubactions:** Update build product name.


<a name="v0.6.2"></a>
## [v0.6.2](https://github.com/sdttttt/gcr/compare/v0.6.1...v0.6.2) (2020-09-25)

### Docs

* perfect document.
* **cargo.toml:** Add Description information.
* **global:** update name from `gcr` to `grc`.
* **readme:** perfect document.
* **readme:** update name.


<a name="v0.6.1"></a>
## [v0.6.1](https://github.com/sdttttt/gcr/compare/v0.6.0...v0.6.1) (2020-09-25)

### Feat

* **cli:** Add Description information. (-h)


<a name="v0.6.0"></a>
## [v0.6.0](https://github.com/sdttttt/gcr/compare/v0.5.5...v0.6.0) (2020-09-25)

### Optimize

* **message.rs:** It's beautiful!

### Style

* **code:** better!

### Pull Requests

* Merge pull request [#1](https://github.com/sdttttt/gcr/issues/1) from sdttttt/sdttttt-patch-1


<a name="v0.5.5"></a>
## [v0.5.5](https://github.com/sdttttt/gcr/compare/v0.5.4...v0.5.5) (2020-09-24)


<a name="v0.5.4"></a>
## [v0.5.4](https://github.com/sdttttt/gcr/compare/v0.5.3...v0.5.4) (2020-09-24)


<a name="v0.5.3"></a>
## [v0.5.3](https://github.com/sdttttt/gcr/compare/v0.5.2...v0.5.3) (2020-09-24)


<a name="v0.5.2"></a>
## [v0.5.2](https://github.com/sdttttt/gcr/compare/v0.5.1...v0.5.2) (2020-09-24)


<a name="v0.5.1"></a>
## [v0.5.1](https://github.com/sdttttt/gcr/compare/v0.5.0...v0.5.1) (2020-09-24)


<a name="v0.5.0"></a>
## [v0.5.0](https://github.com/sdttttt/gcr/compare/v0.4.2...v0.5.0) (2020-09-24)

### Chore

* **output:** update information.

### Ci

* **matrix:** Add Windows Build.
* **upload:** Update upload actions component.

### Feat

* **input:** Begin to be compatible with 'Windows'.


<a name="v0.4.2"></a>
## [v0.4.2](https://github.com/sdttttt/gcr/compare/v0.4.1...v0.4.2) (2020-09-21)

### Fix

* **print:** enhance output.


<a name="v0.4.1"></a>
## [v0.4.1](https://github.com/sdttttt/gcr/compare/v0.4.0...v0.4.1) (2020-09-21)

### Fix

* **ci:** upload release problom.


<a name="v0.4.0"></a>
## [v0.4.0](https://github.com/sdttttt/gcr/compare/v0.3.0...v0.4.0) (2020-09-21)

### Fix

* **main:** noifty of "IF NOT REPO"


<a name="v0.3.0"></a>
## [v0.3.0](https://github.com/sdttttt/gcr/compare/v0.2.0...v0.3.0) (2020-09-20)


<a name="v0.2.0"></a>
## [v0.2.0](https://github.com/sdttttt/gcr/compare/v0.1.0...v0.2.0) (2020-09-20)

### Feat

* add workflow
* **all:** I'm ready!

### Style

* enhance output.


<a name="v0.1.0"></a>
## v0.1.0 (2020-09-20)

### Fead

* Add exector.

### Feat

* complete main function.
* Add commit function to repo mod.
* Add sign function to repo mod.
* 🎸 Complete show_type
* 🎸 Add show type
* **main:** Complete input function.
* **main:** Command execute successed.
* **project:** initialzer project.
* **util:** Add current_path function to util mod.

### Fix

* Commit bug.


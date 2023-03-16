use maomi::{prelude::*, locale_string::*};
use maomi_dom::{prelude::*, element::*};

use crate::{PageMeta, components::utils::code_segment::*};
use super::{GuideWrapper, section, section_title, section_desc};

i18n_group!(guide as trans);

stylesheet!(
    use crate::*;
);

#[component(Backend = DomBackend, Translation = guide)]
pub(crate) struct Content {
    template: template! {
        <GuideWrapper cur_chapter="/guide/i18n-support">
            <div class:section>
                <h2 class:section_title> "Enable i18n support" </h2>
                <p class:section_desc>
                    r#"Maomi can generate different binaries for different languages, a.k.a. the i18n support."#
                </p>
                <p class:section_desc>
                    r#"By default, i18n support is disable. To enable it, specify the locale in "MAOMI_I18N_LOCALE" environment variable while compilation."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"MAOMI_I18N_LOCALE=zh_CN"# />
                </_>
                <p class:section_desc>
                    r#"When this environment variable often changes, it is recommended to add this line to build.rs:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"fn main() {"# />
                    <CodeLine em=&{Emphasize} text=r#"    println!("cargo:rerun-if-env-changed=MAOMI_I18N_LOCALE");"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Translation files" </h2>
                <p class:section_desc>
                    r#"When i18n support enabled, maomi requires every text node in the template to be translated."#
                </p>
                <p class:section_desc>
                    r#"Every string without translation will be marked a compilation error."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Wrong} text=r#"        <div> "My Website" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"To translate this string, create a "i18n/[LOCALE].toml" file. Use "i18n/zh_CN.toml" as an example:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"[translation]"# />
                    <CodeLine text=r#""My Website" = "我的网站""# />
                </_>
                <p class:section_desc>
                    r#"The string will be replaced according to this file when compiling the "MAOMI_I18N_LOCALE=zh_CN" version."#
                </p>
            </div>

            <div class:section>
                <h2 class:section_title> "Dynamic string translation" </h2>
                <p class:section_desc>
                    r#"If the text node is an expression, the value must be a "LocaleString" or "LocaleStaticStr". Normal strings are not accepted."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Wrong} text=r#"        <div> { "My Website" } </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"If the string is static and can be translated through translation files, the `i18n!` macro can be used to translate it."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div> { i18n!("My Website") } </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"If the string is dynamic content or does not need to be translated, mark it as translated manually."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"use maomi::locale_string::LocaleString;"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div> { LocaleString::translated("My Website") } </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Translation groups" </h2>
                <p class:section_desc>
                    r#"By default, the "[translation]" group in translation files are used. It is able to use other groups."#
                </p>
                <CodeSegment>
                    <CodeLine em=&{Emphasize} text=r#"#[component(Backend = DomBackend, Translation = my_group)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine text=r#"        <div> "My Website" </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
                <p class:section_desc>
                    r#"Then in the translation file:"#
                </p>
                <CodeSegment>
                    <CodeLine em=&{Emphasize} text=r#"[my_group]"# />
                    <CodeLine text=r#""My Website" = "我的网站""# />
                </_>
                <p class:section_desc>
                    r#"For dynamic string translation, use "i18n_group!" to get the group."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"i18n_group!(my_group as trans);"# />
                    <CodeLine text=r#""# />
                    <CodeLine text=r#"#[component(Backend = DomBackend)]"# />
                    <CodeLine text=r#"struct MyWebsite {"# />
                    <CodeLine text=r#"    template: template! {"# />
                    <CodeLine em=&{Emphasize} text=r#"        <div> { trans!("My Website") } </div>"# />
                    <CodeLine text=r#"    }"# />
                    <CodeLine text=r#"}"# />
                </_>
            </div>

            <div class:section>
                <h2 class:section_title> "Format tools for translation files" </h2>
                <p class:section_desc>
                    r#"The "maomi-tools" crate provides a tool "maomi-i18n-format" to collect untranslated strings and format the translation files."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"cargo install maomi-tools"# />
                </_>
                <p class:section_desc>
                    r#"Collect the metadata with "MAOMI_I18N_LOCALE" and "MAOMI_I18N_FORMAT_METADATA" environment variable."#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"MAOMI_I18N_LOCALE=zh_CN MAOMI_I18N_FORMAT_METADATA=on cargo check"# />
                </_>
                <p class:section_desc>
                    r#"Then do the format:"#
                </p>
                <CodeSegment>
                    <CodeLine text=r#"MAOMI_I18N_LOCALE=zh_CN maomi-i18n-format"# />
                </_>
            </div>
        </GuideWrapper>
    },
}

impl Component for Content {
    fn new() -> Self {
        Self {
            template: Default::default(),
        }
    }
}

#[async_trait]
impl PrerenderableComponent for Content {
    type QueryData = ();
    type PrerenderingData = ();

    async fn prerendering_data(_: &Self::QueryData) -> Self::PrerenderingData {
        ()
    }

    fn apply_prerendering_data(&mut self, _: Self::PrerenderingData) {
        ()
    }
}

impl PageMeta for Content {
    fn title(&self) -> LocaleString {
        trans!("{} - {}", trans!("maomi Guide"), trans!("I18n Support")).to_locale_string()
    }
}

name = xxx
version = yyy

mode = blog
theme = default


{folder}
// Resource folder, overwritten when inherited
assets = [assets, asset, resources, resource]
// Setting folder, merge on inheritance
config = [configs, config, settings, setting]
// Global variable folder, overwritten when inherited
global = [data, globals, global]
// Template folder, overwritten when inherited
layout = [layouts, layout, templates, template]
// Internationalism folder, overwritten when inherited
locale = [locales, locale, languages, language, i18n]
// Article folder, not copied when inherited
source = [sources, source, articles, article, posts, post, src]
// Plugin folder, merge on inheritance
plugin = [plugins, plugin]

{watch}
{.math}
engine = katex

{.highlight}
engine = prism

{build}
{.math}
engine = katex
{.highlight}
engine = prism
{contributes}
    [.languages]
      ^ id = 'arc'
        aliases = ['ARC'],
        extensions = ['.arc']
        filenames = [ ]
        mimetypes = ['text/x-arc']
        configuration = './syntax/arc.configuration.json'
    [.grammars]
      ^ language = 'arc'
        scopeName = 'source.arc'
        path = './syntax/arc.tmLanguage.json'
      ^ scopeName = 'markdown.arc.codeblock'
        path = './syntax/arc.markdown.json'
        injectTo = ['text.html.markdown']
        embeddedLanguages = {'meta.embedded.block.arc' = 'arc'}
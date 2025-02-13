export type CssVariable = {
  name: string
  value: string
}

export const cssVariables = (filter?: RegExp) => {
  const styleSheets = Array.from(document.styleSheets)

  const variables: Array<CssVariable> = []

  styleSheets.forEach(styleSheet => {
    if (styleSheet.href === null) {
      const rules = Array.from(styleSheet.cssRules)
      rules.forEach(rule => {
        if (rule instanceof CSSStyleRule) {
          const styles = Array.from(rule.style)
          styles.forEach(style => {
            if (style.startsWith('--')) {
              if (filter && !filter.test(style)) return
              variables.push({
                name: style,
                value: rule.style.getPropertyValue(style),
              })
            }
          })
        }
      })
    }
  })

  return variables
}

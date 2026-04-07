# Spliq

Spliq aims to make UI writing styles, renderers, development tools interchangeable, so that changing only one part doesn't require throwing away the rest of your codes or ecosystem.

## Why Spliq

In GUI libraries, the way UI is written and the surrounding ecosystem are very often tightly coupled.

For example, in web development, when a developer working within an ecosystem that is not signal-based wants to adopt a signal-based style of UI programming, it is usually difficult to simply swap out the syntax alone. In practice, this often means considering a migration of the framework and its surrounding assets as well. While this is not 100% impossible, it is still difficult enough that migration is often the practical outcome.

This kind of tight coupling has its advantages. It makes it easier to optimize the system as a whole and to pursue higher performance.  
However, on modern computers, many UI applications can achieve sufficiently practical performance without requiring maximally efficient rendering or extreme optimization at all times.

If that is the case, then it is more reasonable not to tie together UI writing style, renderer, runtime environment, and surrounding tools more tightly than necessary. Instead, they should be separated so that changing only one part doesn't require throwing away the rest of your codes or ecosystem.

Spliq is designed to pursue that kind of separable and interchangeable UI ecosystem.

## Naming Conventions

Spliq ecosystem crates should follow consistent naming conventions so that their roles are immediately clear.

Names are based on the roles defined by Spliq Core. See [Spliq Core](https://github.com/naoshinn/spliq-core) for details.

- Crates that implement `Renderer` should be named `Spliq [Name] Renderer`
- Crates that provide a runtime implementation should be named `Spliq [Name] Runtime`
- Crates that provide a UI writing style should be named `Spliq [Name] Style`
- Other ecosystem projects may be named `Spliq [Name]`

These names are intended to clearly indicate the role of a project within the Spliq ecosystem.  
To avoid confusion, projects or crates that do not actually provide the corresponding role should avoid using them.

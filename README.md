# Leptos conditional router outlet MRE
This crate shows an error thrown when using a conditionally rendered `<Outlet/>`.

When clicking the `Show outlet` button, the error should appear.
```shell
Tried to render <Outlet/> but could not find the view receiver. Are you using the same <Outlet/> twice?
```

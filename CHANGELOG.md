# Changelog

## 2.0.0 (2020-06-03)

* The `tico` function now receives two arguments. The first argument is still the current path, and the second argument is an option with the home directory. If a home directory is passed, `tico` will replace it in the path with `~`.
* The `tico` binary now automatically shortens the home directory in the path to `~`.
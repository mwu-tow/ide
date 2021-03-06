# Next Release
This is a minor update exposing more control over the anonymous data collection. You are now allowed 
to run the application with `--no-errors-data-collection`, and `--no-data-collection`. The options 
disable the collection of errors data (keeping the collection of other events), and disables the 
collection of all data, respectively. To learn more what data is collected in Enso alpha releases, 
read the notes of the `Enso 2.0.0-alpha.1` release.

<br/>![Bug Fixes](/docs/assets/tags/bug_fixes.svg)

#### Visual Environment
- Pressing and holding up and down arrow keys make the list view selection to move multiple times.
- Cursors in text editors behave correctly now (they are not affected by scene pan and zoom). This
  was possible because of a new multi-camera management system implemented in EnsoGL.
- Fixes to some visual glitches, like small "pixel-like" things appearing sometimes on the screen.
- The shortcuts to close the application and to toggle the developer tools at runtime are working
  now on all supported platforms.
- [You can now see data frames in the table visualisation][1181]. Big tables get truncated to 2000 
  entries.
- [Documentation in Searcher][1098]. Fixed cases where text would be misinterpreted as tag, added
  support for new tag types, added support for more common characters, properly renders overflowing
  text.
- [Assigning intermediate expressions to values][1067]. Nodes added with searcher will have their 
  values automatically assigned to a newly generated variables. This allows Enso Engine to cache
  intermediate values, improving visualization performance.
- [Improved handling of projects created with other IDE versions][1214]. IDE is not better at 
  dealing with incompatible node metadata (that store node visual positions, history of picked 
  searcher suggestions, etc.). This will allow IDE to correctly open projects that were created 
  using a different IDE version and prevent unnecessary lose of metadata information.
  
#### EnsoGL
- New multi-camera management system, allowing the same shape systems be rendered on different 
  layers from different cameras. The implementation automatically caches the same shape system
  definitions per scene layer in order to minimize the amount of WebGL draw calls.
- New symbols and shapes depth ordering mechanism. It is now possible to define depth order 
  dependencies between symbols, shapes, and shape systems.
- Various performance improvements, especially for the text rendering engine.
- Display objects handle visibility correctly now. Display objects are not visible by default and 
  need to be attached to a visible parent to be shown on the screen.

<br/>![New Features](/docs/assets/tags/new_features.svg)

#### Visual Environment
- [Added the ability to reposition visualisations.][1096] There is now an icon in the visualization 
  action bar that allows dragging the visualization. Once the visualization has been moved, there 
  appears another icon that will reset the position to the original position.
- [Allow Tables to feed the Geo Map visualisation.][1187] Tables that have `latitude`, `longitude`
  and optionally `label` columns can now be shown in a Geo Map visualisation where each row is 
  mapped to a point of the map with the given label.
- [Erroneous nodes are highlighted.][1182] The error description appears above the node. The nodes 
  affected by error originating from another node will be highlighted without any description.
- [There is now an API to show VCS status for node][1160].
- [A shortcut for reloading visualization files.][1190] The visible visualizations must be switched 
  to another and switched back to see the effect.

[1096]: https://github.com/enso-org/ide/pull/1172
[1098]: https://github.com/enso-org/ide/pull/1098
[1181]: https://github.com/enso-org/ide/pull/1181
[1182]: https://github.com/enso-org/ide/pull/1182
[1160]: https://github.com/enso-org/ide/pull/1160
[1190]: https://github.com/enso-org/ide/pull/1190
[1187]: https://github.com/enso-org/ide/pull/1187
[1214]: https://github.com/enso-org/ide/pull/1214
<br/>


# Enso 2.0.0-alpha.1 (2020-01-26)
This is the first release of Enso, a general-purpose programming language and environment for 
interactive data processing. It is a tool that spans the entire stack, going from high-level 
visualization and communication to the nitty-gritty of backend services, all in a single language.

<br/>![Release Notes](/docs/assets/tags/release_notes.svg)

#### Anonymous Data Collection
Please note that this release collects anonymous usage data which will be used to improve Enso and 
prepare it for a stable release. We will switch to opt-in data collection in stable version 
releases. The usage data will not contain your code (expressions above nodes), however, reported 
errors may contain brief snippets of out of context code that specifically leads to the error, like 
"the method 'foo' does not exist on Number". The following data will be collected:
- Session length.
- Graph editing events (node create, dele, position change, connect, disconnect, collapse, edit 
  start, edit end). This will not include any information about node expressions used.
- Navigation events (camera movement, scope change).
- Visualization events (visualization open, close, switch). This will not include any information 
  about the displayed data nor the rendered visualization itself.
- Project management events (project open, close, rename).
- Errors (IDE crashes, WASM panics, Project Manager errors, Language Server errors, Compiler 
  errors).
- Performance statistics (minimum, maximum, average GUI refresh rate).

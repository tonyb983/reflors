<!--
 Copyright (c) 2022 Tony Barbitta
 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

# reflo-rs (like re-floors?)

Shittily porting the golib [`reflow`](https://github.com/muesli/reflow) to rust!

Some lingering thoughts after reading through this go library more...
- First, in the first module, `ansi`, there is an implementation of an ansi-aware buffer, but why isn't this buffer used in any of the other modules? In the padding and the truncate modules, he implements another variation of a writer, but uses a `bytes.Buffer` and manually counts the visible width. Why not just use the ansi aware buffer thats already in the library?? It makes no sense...
- In the `padding` module, for some reason only end of string padding is implemented, this also doesn't make sense and will be on my todolist of things to upgrade after I have the basic implementation completed. 
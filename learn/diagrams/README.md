# Diagrams

The diagrams are [Excalidraw](https://excalidraw.com) files: plain JSON, so they diff like code and
you can edit them. Only the `.excalidraw` sources live in git, not image exports. `design.md` has a
text version of each one, so you can read it in a terminal.

| File | Shows | Text version |
|---|---|---|
| `actors.excalidraw` | Who talks to whom | `design.md` §1 |
| `tool-call.excalidraw` | One tool call, end to end | `design.md` §2 |
| `layers.excalidraw` | The layers, and which milestone builds each | `design.md` §3 |

## Opening them

**In the browser (online, or offline once cached):** go to <https://excalidraw.com>, then
menu → *Open* (or `Ctrl+O`) and pick the file. Excalidraw works offline after your first visit:
visit it once while you're online.

**Offline with Podman:** pull the image while you have internet:

```sh
podman pull docker.io/excalidraw/excalidraw:latest
```

Then, whenever you need it:

```sh
podman run --rm -d --name excalidraw -p 127.0.0.1:5000:80 docker.io/excalidraw/excalidraw:latest
# open http://localhost:5000, then menu → Open → pick the file
podman stop excalidraw
```

## Editing

Open the file, change it, then menu → *Save to…* over the same `.excalidraw` file (in Chromium-based
browsers; in Firefox, *Save* downloads a copy, so move it over the original). Keep the text version
in `design.md` in sync if the meaning changes. Add new diagrams the same way, and put the drawings
from your own milestone work here too.

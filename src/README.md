
# Source Code Structure

The source code is made up of three main sections that each process one stage of the pipeline to produce the final documentation webpages.

## Parser:

The parser is a section that processes each of the files that may have a block and formats them for the next process.

## Renderer:

This stage takes from the AnubisDatabase any parsed blocks and processes them into three individual data structures:

- Connections Graph:

  An undirected graph data structure backed with PetGraph allows us to create both a list of neighbors within the page, and a full Zettlekasten

- Html Page Content:

  The rendered main content for the page to be included within the center.

- Header List:

  A list of the markdown headers contained within the page allows us to build a list of inner page links.

## Server:

The server takes a provided URI and then will dynamically render a template with the information stored from rendering and send this to the client.

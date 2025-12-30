
## Extract common logic from match statements (ECLFMS)

Here, we have a match statement with duplicated stuff:

```scala
    // If --input_vpst is provided, load .vpst files
    // The Rust parser already filtered to only needed packages via import-driven parsing
    val (allInputs, packageCoords) = opts.inputVpstDir match {
      case Some(vpstDir) => {
        val vpstFiles = new java.io.File(vpstDir).listFiles().filter(_.getName.endsWith(".vpst"))
        val vpstInputs = vpstFiles.map(file => {
          val code = Source.fromFile(file).mkString
          val fileP = parseStuff(code)
          val packageCoord = fileP.fileCoord.packageCoordinate
          SourceInput(packageCoord, file.getPath, code)
        }).toVector
        val coords = vpstInputs.map(_.packageCoord(interner)).distinct
        (opts.inputs ++ vpstInputs, coords)
      }
      case None => (opts.inputs, opts.inputs.map(_.packageCoord(interner)).distinct)
    }




    // If --input_vpst is provided, load .vpst files
    // The Rust parser already filtered to only needed packages via import-driven parsing
    val allInputs = opts.inputVpstDir match {
      case Some(vpstDir) => {
        val vpstFiles = new java.io.File(vpstDir).listFiles().filter(_.getName.endsWith(".vpst"))
        val vpstInputs = vpstFiles.map(file => {
          val code = Source.fromFile(file).mkString
          val parsedLoader = new ParsedLoader(interner)
          val fileP = parsedLoader.load(code) match {
            case Err(e) => return Err(s"Failed to load ${file.getName}: $e")
            case Ok(f) => f
          }
          val packageCoord = fileP.fileCoord.packageCoordinate
          SourceInput(packageCoord, file.getPath, code)
        }).toVector
        opts.inputs ++ vpstInputs
      }
      case None => opts.inputs
    }

    val packageCoords = allInputs.map(_.packageCoord(interner)).distinct

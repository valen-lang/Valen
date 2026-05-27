#include "ast.h"
#include "../utils/rustify.h"

extern const std::string BUILTIN_PROJECT_NAME = "__vale";

std::string Package::getKindExportName(Kind* kind, bool includeProjectName) const {
  if (auto innt = dynamic_cast<Int *>(kind)) {
    return std::string() + "int" + std::to_string(innt->bits) + "_t";
  } else if (dynamic_cast<Bool *>(kind)) {
    return "int8_t";
  } else if (dynamic_cast<Float *>(kind)) {
    return "double";
  } else if (dynamic_cast<Str *>(kind)) {
    return "ValeStr*";
  } else if (auto opaque = dynamic_cast<Opaque *>(kind)) {
    auto iter2 = kindToExtern.find(opaque);
    if (iter2 == kindToExtern.end()) {
      std::cerr << "Couldn't find export name for: " << getKindHumanName(kind) << std::endl;
      exit(1);
    }
    auto thing = iter2->second->mangledName; // DO NOT SUBMIT name
    return (includeProjectName && !packageCoordinate->projectName.empty() ? packageCoordinate->projectName + "_" : "") + thing;
  } else {
    // DO NOT SUBMIT this is awkward
    std::string thing;
    auto iter1 = kindToExportName.find(kind);
    if (iter1 == kindToExportName.end()) {
      std::cerr << "Couldn't find export name for: " << getKindHumanName(kind) << std::endl;
      exit(1);
    }
    thing = iter1->second;
    return (includeProjectName && !packageCoordinate->projectName.empty() ? packageCoordinate->projectName + "_" : "") + thing;
  }
}
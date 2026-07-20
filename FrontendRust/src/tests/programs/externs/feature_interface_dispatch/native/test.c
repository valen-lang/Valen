#include <stdint.h>
#include <stdlib.h>

#include "vtest/IShape.h"
#include "vtest/Circle.h"
#include "vtest/Square.h"
#include "vtest/Triangle.h"
#include "vtest/IShape_alias.h"
#include "vtest/IShape_dealias.h"
#include "vtest/IShape_typeTag.h"
#include "vtest/IShape_asCircle.h"
#include "vtest/IShape_asSquare.h"
#include "vtest/IShape_asTriangle.h"
#include "vtest/Circle_alias.h"
#include "vtest/Circle_dealias.h"
#include "vtest/Circle_radius.h"
#include "vtest/Square_alias.h"
#include "vtest/Square_dealias.h"
#include "vtest/Square_side.h"
#include "vtest/Triangle_alias.h"
#include "vtest/Triangle_dealias.h"
#include "vtest/Triangle_base.h"
#include "vtest/Triangle_height.h"

// Per @FRMACZ: alias each handle at every pass into an accessor, and dealias
// each handle we own once we're done.
ValeInt vtest_computeArea(vtest_IShape s) {
  ValeInt result = 0;
  switch (vtest_IShape_typeTag(vtest_IShape_alias(s))) {
    case vtest_IShape_TAG_Circle: {
      vtest_Circle c = vtest_IShape_asCircle(vtest_IShape_alias(s));
      ValeInt r = vtest_Circle_radius(vtest_Circle_alias(c));
      vtest_Circle_dealias(c);
      result = 3 * r * r;
      break;
    }
    case vtest_IShape_TAG_Square: {
      vtest_Square sq = vtest_IShape_asSquare(vtest_IShape_alias(s));
      ValeInt side = vtest_Square_side(vtest_Square_alias(sq));
      vtest_Square_dealias(sq);
      result = side * side;
      break;
    }
    case vtest_IShape_TAG_Triangle: {
      vtest_Triangle t = vtest_IShape_asTriangle(vtest_IShape_alias(s));
      ValeInt base = vtest_Triangle_base(vtest_Triangle_alias(t));
      ValeInt height = vtest_Triangle_height(vtest_Triangle_alias(t));
      vtest_Triangle_dealias(t);
      result = base * height / 2;
      break;
    }
    default:
      exit(1);
  }
  vtest_IShape_dealias(s);
  return result;    // 6 * 14 / 2 == 42
}

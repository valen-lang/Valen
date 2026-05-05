//
// Created by Evan Ovadia on 6/3/24.
//

#ifndef BACKEND_RUSTIFY_H
#define BACKEND_RUSTIFY_H

#include "../metal/ast.h"
#include <string>

std::string rustifySimpleId(SimpleId* simpleId, bool ignoreFirst);



#endif //BACKEND_RUSTIFY_H

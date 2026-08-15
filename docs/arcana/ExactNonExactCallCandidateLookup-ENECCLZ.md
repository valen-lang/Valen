# Exact/Non-Exact Call Candidate Lookup (ENECCLZ)

Overload lookup (`find_function`, `find_potential_function`) runs in one of two modes, chosen by an `exact: bool` that reaches both `get_param_environments` and `params_match`.

**Non-exact** is a user call site, like `launch(ship)`. It peels the argument's references to pick which namespace to search, so a `&Ship` argument means searching `Ship`'s environment. It also considers subtypes/supertypes, so calling something on a &Ship should also look in IFlying's namespaces.

**Exact** is the compiler resolving a function whose shape it already knows: a `where`-clause bound, or a virtual override. It keeps the type whole, so a `&Ship` searches the borrow reference's environment (`borrow.vale`) rather than `Ship`'s; and it matches parameters by equality, with no coercion.

The split is important for e.g. a callsite trying to satisfy a callee's bound `where exists clone(&T)T`.
 * If the caller is handing in `T = Ship`, it should search `Ship`'s environment and finds a hand-written `clone(&Ship)Ship`.
 * If the caller is handing in `T = &Ship`, it should search `&Ship`'s environment, specifically `borrow.vale`, and find the blanket `clone<T>(&&T)&T` function in there.

Unconditionally peeling the reference would be the wrong move.
Searching in all environments would also be the wrong move.

Virtual dispatch also needs it, because vtables can't do any casting, they need to contain function pointers for the *exact* function they expect.

Both of these are handled by `exact: bool`. It's TBD whether we'll want to split that into two booleans, unsure. For now they're one.

VCOORD: is this true still?
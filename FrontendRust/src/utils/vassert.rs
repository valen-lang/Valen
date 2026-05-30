/*
package dev.vale

import scala.annotation.elidable

// We use this instead of a regular RuntimeException because ScalaTest likes to print out
// theException.getMessage then theException.toString then theException.printStackTrace().
// Since RuntimeException's getMessage and toString both return the message, it means
// ScalaTest prints out its message twice. Rather irksome to see a giant error message twice
// when debugging test failures.
class VAssertionFailException(message: String) extends RuntimeException {
  override def getMessage: String = message
  override def toString: String = "Assertion failed! " + message
}

// A condition that reflects a user error.
object vcheck {
  def apply[T <: Throwable](condition: Boolean, message: String): Unit = {
    vcheck(condition, message, (s) => new RuntimeException(s))
  }
  def apply[T <: Throwable](condition: Boolean, exceptionMaker: (String) => T): Unit = {
    vcheck(condition, "Check failed!", exceptionMaker)
  }
  def apply[T <: Throwable](condition: Boolean, exceptionMaker: () => Throwable): Unit = {
    if (!condition) {
      throw exceptionMaker()
    }
  }
  def apply[T <: Throwable](condition: Boolean, message: String, exceptionMaker: (String) => T): Unit = {
    if (!condition) {
      throw exceptionMaker(message)
    }
  }
}

// A condition that reflects a programmer error.
object vassert {
  @elidable(elidable.OFF)
  def apply(condition: => Boolean): Boolean = {
    vassert(condition, "Assertion failed!")
  }
  @elidable(elidable.OFF)
  def apply(condition: => Boolean, message: => String): Boolean = {
    if (!condition) {
      vfail(message)
      true
    } else {
      false
    }
  }
}

object vcurious {
  def apply(): Nothing = {
    vfail()
  }
  def apply(message: String): Nothing = {
    vfail(message)
  }
  def apply(condition: Boolean): Unit = {
    vassert(condition)
  }
  def apply(condition: Boolean, message: String): Unit = {
    vassert(condition, message)
  }
}


object vassertSome {
  def apply[T](thing: Option[T], message: String): T = {
    thing match {
      case None => vfail(message)
      case Some(x) => x
    }
  }
  def apply[T](thing: Option[T]): T = {
    apply(thing, "Expected non-empty!")
  }
}
*/
// The other v-helpers above (vcheck/vassert/vcurious/vassertSome) and below (vfail/vwat/vimpl/
// vregion/vregionmut) are realized inline in Rust as `assert!`/`panic!`/`.expect()` at their call
// sites (per migration-policy), so they have no Rust fn here. `vassertOne` is the exception: it
// returns the sole element, which has no built-in idiom, so it's a real helper.
// mig: fn vassert_one
pub fn vassert_one<T>(thing: impl IntoIterator<Item = T>) -> T {
    let mut iter = thing.into_iter();
    match iter.next() {
        None => panic!("Expected one element, but was empty."),
        Some(x) => {
            let extra = iter.count();
            if extra == 0 {
                x
            } else {
                panic!("Expected one element, but was size {}.", extra + 1)
            }
        }
    }
}
/*
object vassertOne {
  def apply[T](thing: Iterable[T], message: String): T = {
    thing.toList match {
      case List(x) => x
      case _ => vfail(message)
    }
  }
  def apply[T](thing: Iterable[T]): T = {
    thing.toList match {
      case List() => vfail("Expected one element, but was empty.")
      case List(x) => x
      case other => vfail("Expected one element, but was size " + other.size + ".")
    }
  }
  def apply[T](thing: Vector[T], message: String): T = {
    thing.toList match {
      case List(x) => x
      case _ => vfail(message)
    }
  }
  def apply[T](thing: Vector[T]): T = {
    apply(thing, "Expected exactly one element!")
  }
}
*/
/*
object vfail {
  def apply(message: Object): Nothing = {
    throw new VAssertionFailException(message.toString)
  }
  def apply(): Nothing = {
    vfail("fail!")
  }
}

object vwat {
  def apply(): Nothing = {
    vfail("wat")
  }
  def apply(message: Object*): Nothing = {
    vfail("wat: " + message.toVector.mkString(", "))
  }
}

object vimpl {
  def apply(): Nothing = {
    vfail("impl")
  }
  def apply(message: Object): Nothing = {
    vfail(message.toString)
  }

  def unapply(thing: Any): Option[Nothing] = {
    vimpl()
  }
}

// this is mainly a passthrough, and marks something that needs to be implemented or doublechecked
// for region support
object vregion {
  def apply[T](obj: T): T = {
    vimpl()
    obj
  }
}

// this is mainly a passthrough, and marks something that needs to be implemented or doublechecked
// for mutable/immutable region support
object vregionmut {
  def apply[T](obj: T): T = {
    obj
  }
}
*/

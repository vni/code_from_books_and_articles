## Chapter 1

** **
Keep in mind that interior mutability only bends the rules of shared borrowing to allow mutation when shared. It does not change anything about exclusive borrowing. Exclusive borrowing still guarantees that there are no other active borrows. Unsafe code that results in more than one active exclusive reference to something always invokes undefined behavior, regardless of interior mutability.

** Cell **
    A std::cell::Cell<T> simply wraps a T, but allows mutations through a shared reference. To avoid undefined behavior, it only allows you to copy the value out (if T is Copy), or replace it with another value as a whole. In addition, it can only be used within a single thread.

** RefCell **
    Unlike a regular Cell, a std::cell::RefCell does allow you to borrow its contents, at a small runtime cost. A RefCell<T> does not only hold a T, but also holds a counter that keeps track of any outstanding borrows. If you try to borrow it while it is already mutably borrowed (or vice-versa), it will panic, which avoids undefined behavior. Just like a Cell, a RefCell can only be used within a single thread.

** UnsafeCell **
    An UnsafeCell<T> wraps a T, but does not come with any conditions or restrictions to avoid undefined behavior. Instead, its get() method just gives a raw pointer to the value it wraps, which can only be meaningfully used in unsafe blocks. It leaves it up to the user to use it in a way that does not cause any undefined behavior.

** RwLock **
     An RwLock or reader-writer lock is the concurrent version of a RefCell. An RwLock<T> holds a T and tracks any outstanding borrows. However, unlike a RefCell, it does not panic on conflicting borrows. Instead, it blocks the current thread—​putting it to sleep—​while waiting for conflicting borrows to disappear. We’ll just have to patiently wait for our turn with the data, after the other threads are done with it.
     Borrowing the contents of an RwLock is called locking. By locking it we temporarily block concurrent conflicting borrows, allowing us to borrow it without causing data races.

** Send **
    A type is Send if it can be sent to another thread. In other words, if ownership of a value of that type can be transferred to another thread. For example, Arc<i32> is Send, but Rc<i32> is not.


** Sync **
    A type is Sync if it can be shared with another thread. In other words, a type T is Sync if and only if a shared reference to that type, &T, is Send. For example, an i32 is Sync, but a Cell<i32> is not. (A Cell<i32> is Send, however.)

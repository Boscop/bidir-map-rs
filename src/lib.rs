use std::borrow::Borrow;
use std::slice;
use std::iter::{Extend, FromIterator};
use std::vec;


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BidirMap<Kv1: PartialEq, Kv2: PartialEq> {
	cont: Vec<(Kv1, Kv2)>,
}

impl<Kv1: PartialEq, Kv2: PartialEq> BidirMap<Kv1, Kv2> {
	/// Create a new empty instance of `BidirMap`
	pub fn new() -> Self {
		BidirMap{
			cont: Vec::new(),
		}
	}

	/// Clears the map, removing all entries.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// a.insert(1, "a");
	/// a.clear();
	/// assert!(a.is_empty());
	/// ```
	pub fn clear(&mut self) {
		self.cont.clear()
	}

	/// Inserts a K/V-K/V pair into the map.
	///
	/// If the map did not have this K/V-K/V pair present, `None` is returned.
	///
	/// If the map did have this K/V-K/V pair present, it's updated and the old K/V-K/V pair is returned.
	pub fn insert(&mut self, kv1: Kv1, kv2: Kv2) -> Option<(Kv1, Kv2)> {
		let retval =
			if self.contains_first_key(&kv1) {
				self.remove_by_first(&kv1)
			} else if self.contains_second_key(&kv2) {
				self.remove_by_second(&kv2)
			} else {
				None
			};

		self.cont.push((kv1, kv2));

		retval
	}

	/// Gets an iterator over the entries of the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// map.insert(2, "b");
	/// map.insert(3, "c");
	///
	/// for kv in map.iter() {
	/// 	println!("{}: {}", kv.0, kv.1);
	/// }
	///
	/// let first = map.iter().next().unwrap();
	/// assert_eq!(*first, (1, "a"));
	/// ```
	pub fn iter<'a>(&'a self) -> slice::Iter<'a, (Kv1, Kv2)> {
		self.cont.iter()
	}

	/// Gets a mutable iterator over the entries of the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert("a", 1);
	/// map.insert("b", 2);
	/// map.insert("c", 3);
	///
	/// // add 10 to the value if the key isn't "a"
	/// for kv in map.iter_mut() {
	/// 	if &kv.0 != &"a" {
	/// 		kv.1 += 10;
	/// 	}
	/// }
	/// ```
	pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, (Kv1, Kv2)> {
		self.cont.iter_mut()
	}

	//TODO: maybe implement keys() and values() as first_row() and second_row()?

	/// Returns the number of elements in the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// assert_eq!(a.len(), 0);
	/// a.insert(1, "a");
	/// assert_eq!(a.len(), 1);
	/// ```
	pub fn len(&self) -> usize {
		self.cont.len()
	}

	/// Returns true if the map contains no elements.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// assert!(a.is_empty());
	/// a.insert(1, "a");
	/// assert!(!a.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.cont.is_empty()
	}


	/// Returns a reference to the first K/V corresponding to the second K/V.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map: BidirMap<i32, &'static str> = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get_by_first(&1), Some(&"a"));
	/// assert_eq!(map.get_by_first(&2), None);
	/// ```
	pub fn get_by_first<Q>(&self, key: &Q) -> Option<&Kv2>
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter().find(|&kvs| *key == kvs.0).map(|ref kvs| &kvs.1)
	}

	/// Returns a reference to the first K/V corresponding to the second K/V.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map: BidirMap<i32, &'static str> = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get_by_second(&"a"), Some(&1));
	/// assert_eq!(map.get_by_second(&"b"), None);
	/// ```
	pub fn get_by_second<Q>(&self, key: &Q) -> Option<&Kv1>
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter().find(|&kvs| *key == kvs.1).map(|ref kvs| &kvs.0)
	}

	/// Check if the map contains the first K/V
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map: BidirMap<i32, &'static str> = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.contains_first_key(&1), true);
	/// assert_eq!(map.contains_first_key(&2), false);
	/// ```
	pub fn contains_first_key<Q>(&self, key: &Q) -> bool
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter().any(|ref kvs| *key == kvs.0)
	}

	/// Check if the map contains the second K/V
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map: BidirMap<i32, &'static str> = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.contains_second_key(&"a"), true);
	/// assert_eq!(map.contains_second_key(&"b"), false);
	/// ```
	pub fn contains_second_key<Q>(&self, key: &Q) -> bool
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter().any(|ref kvs| *key == kvs.1)
	}

	//TODO: implement get_mut
	/*pub fn get_mut_by_first<Q>(&mut self, key: &Q) -> Option<&mut Kv2>
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter_mut().find(|&mut kvs| key == kvs.0).map(|&mut kvs| &mut kvs.1)
	}
	pub fn get_mut_by_second<Q>(&mut self, key: &Q) -> Option<&mut Kv1>
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter_mut().find(|&mut kvs| key == kvs.1).map(|&mut kvs| &mut kvs.0)
	}*/

	/// Removes the pair corresponding to the first K/V from the map, returning it if the key was previously in the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.remove_by_first(&1), Some((1, "a")));
	/// assert_eq!(map.remove_by_first(&1), None);
	/// ```
	pub fn remove_by_first<Q>(&mut self, key: &Q) -> Option<(Kv1, Kv2)>
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter().position(|ref kvs| *key == kvs.0).map(|idx| self.cont.swap_remove(idx))
	}

	/// Removes the pair corresponding to the first K/V from the map, returning it if the key was previously in the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.remove_by_second(&"a"), Some((1, "a")));
	/// assert_eq!(map.remove_by_second(&"b"), None);
	/// ```
	pub fn remove_by_second<Q>(&mut self, key: &Q) -> Option<(Kv1, Kv2)>
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter().position(|ref kvs| *key == kvs.1).map(|idx| self.cont.swap_remove(idx))
	}
}


impl<Kv1: PartialEq, Kv2: PartialEq> IntoIterator for BidirMap<Kv1, Kv2> {
	type Item = (Kv1, Kv2);
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		return self.cont.into_iter()
	}
}

impl<Kv1: PartialEq, Kv2: PartialEq> FromIterator<(Kv1, Kv2)> for BidirMap<Kv1, Kv2> {
	fn from_iter<T: IntoIterator<Item=(Kv1, Kv2)>>(iter: T) -> Self {
		BidirMap{
			cont: Vec::from_iter(iter),
		}
	}
}

impl<Kv1: PartialEq, Kv2: PartialEq> Extend<(Kv1, Kv2)> for BidirMap<Kv1, Kv2> {
	fn extend<T: IntoIterator<Item=(Kv1, Kv2)>>(&mut self, iter: T) {
		self.cont.extend(iter)
	}
}
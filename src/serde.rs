use crate::{Matrix, Vector};

struct InPlaceSeed<'a, T: 'a>(pub &'a mut T);

impl<'a, 'de, T> serde::de::DeserializeSeed<'de> for InPlaceSeed<'a, T>
where
    T: serde::de::Deserialize<'de>,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        T::deserialize_in_place(deserializer, self.0)
    }
}

struct SliceInPlaceSeed<'a, T: 'a>(pub &'a mut [T]);

impl<'a, 'de, T> serde::de::DeserializeSeed<'de> for SliceInPlaceSeed<'a, T>
where
    T: serde::de::Deserialize<'de>,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(self.0.len(), SliceVisitorInPlace(self.0))
    }
}

struct ArrayVisitor<T, const N: usize> {
    marker: core::marker::PhantomData<[T; N]>,
}

impl<T, const N: usize> ArrayVisitor<T, N> {
    fn new() -> Self {
        ArrayVisitor {
            marker: core::marker::PhantomData,
        }
    }
}

impl<'de, T, const N: usize> serde::de::Visitor<'de> for ArrayVisitor<T, N>
where
    T: serde::de::Deserialize<'de>,
{
    type Value = [T; N];

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "an array of length {}", N)
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<[T; N], A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        #![allow(unsafe_code)]

        crate::array_init::try_array_init(|index| match seq.next_element() {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(serde::de::Error::invalid_length(index, &self)),
            Err(err) => Err(err),
        })
    }
}

struct SliceVisitorInPlace<'a, T: 'a>(&'a mut [T]);

impl<'a, 'de, T> serde::de::Visitor<'de> for SliceVisitorInPlace<'a, T>
where
    T: serde::de::Deserialize<'de>,
{
    type Value = ();

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "an array of length {}", self.0.len())
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut fail_idx = None;

        for (idx, dest) in self.0.iter_mut().enumerate() {
            if seq.next_element_seed(InPlaceSeed(dest))?.is_none() {
                fail_idx = Some(idx);

                break;
            }
        }

        if let Some(fail_idx) = fail_idx {
            return Err(serde::de::Error::invalid_length(fail_idx, &self));
        }

        Ok(())
    }
}

struct SliceOfArraysVisitorInPlace<'a, T: 'a, const N: usize>(&'a mut [[T; N]]);

impl<'a, 'de, T, const N: usize> serde::de::Visitor<'de> for SliceOfArraysVisitorInPlace<'a, T, N>
where
    T: serde::de::Deserialize<'de>,
{
    type Value = ();

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "an array of length {}", N)
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut fail_idx = None;

        for (idx, dest) in self.0.iter_mut().enumerate() {
            if seq.next_element_seed(SliceInPlaceSeed(dest))?.is_none() {
                fail_idx = Some(idx);

                break;
            }
        }

        if let Some(fail_idx) = fail_idx {
            return Err(serde::de::Error::invalid_length(fail_idx, &self));
        }

        Ok(())
    }
}

impl<T, const N: usize> serde::Serialize for Vector<T, N>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.array().serialize(serializer)
    }
}

impl<'de, T, const N: usize> serde::de::Deserialize<'de> for Vector<T, N>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        match deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>::new()) {
            Ok(array) => Ok(Vector::from_array(array)),
            Err(err) => Err(err),
        }
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(N, SliceVisitorInPlace(place.array_mut()))
    }
}

impl<T, const N: usize, const M: usize> serde::Serialize for Matrix<T, N, M>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.arrays()
            .each_ref()
            .map(|a| &a[..])
            .serialize(serializer)
    }
}

impl<'de, T, const N: usize, const M: usize> serde::de::Deserialize<'de> for Matrix<T, N, M>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let vectors = deserializer.deserialize_tuple(N, ArrayVisitor::<Vector<T, M>, N>::new())?;
        Ok(Matrix::from_column_arrays(vectors.map(Vector::into_array)))
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(N, SliceOfArraysVisitorInPlace(place.arrays_mut()))
    }
}

enum HeaderFormatValues{
	ZeroSizedFormat = 0, // nil, true false
	NonIndexableWithSlotsFormat = 1, // Point
	IndexableWithoutSlotsFormat  = 2, // Array
	IndexableWithSlotsFormat = 3, // MethodContext 
	WeakIndexableWithSlotsFormat = 4, // Weak Array
	WeakNonIndexableWithSlotsFormat = 5, // Ephemerons
	// 6 is unused
	ImmediateFormat = 7, // Smallinteger, Characters, BoxedFloats
	// 8 is unused
	I64BitIndexable = 9,

	//todo, can we do ranges ? 
}

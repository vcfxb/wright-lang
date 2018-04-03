// todo: module level docs.
// todo: finish opcode docs.

/// Java 9 opcodes.
///
/// Opcodes are also sometimes referred to as 'instructions' or 'byte-codes'.
///
/// Used information from [wikipedia](https://en.wikipedia.org/wiki/Java_bytecode_instruction_listings)
/// and [the java virtual machine 9 specification](https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf).
///
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    /// nop: no operation.
    nop = 0x00,
    /// Push a null reference onto the stack.
    aconst_null = 0x01,
    /// Push the integer value -1 onto the stack.
    iconst_m1 = 0x02,
    /// Push the integer value 0 onto the stack.
    iconst_0 = 0x03,
    /// Push the integer value 1 onto the stack.
    iconst_1 = 0x04,
    /// Push the integer value 2 onto the stack.
    iconst_2 = 0x05,
    /// Push the integer value 3 onto the stack.
    iconst_3 = 0x06,
    /// Push the integer value 4 onto the stack.
    iconst_4 = 0x07,
    /// Push the integer value 5 onto the stack.
    iconst_5 = 0x08,
    /// Push the value 0 onto the stack as a long.
    lconst_0 = 0x09,
    /// Push the value 1 onto the stack as a long.
    lconst_1 = 0x0a,
    /// Push the float value 0.0 onto the stack.
    fconst_0 = 0x0b,
    /// Push the float value 1.0 onto the stack.
    fconst_1 = 0x0c,
    /// Push the float value 2.0 onto the stack.
    fconst_2 = 0x0d,
    /// Push the value 0.0 onto the stack as a double.
    dconst_0 = 0x0e,
    /// Push the value 1.0 onto the stack as a double.
    dconst_1 = 0x0f,
    /// Push a byte value onto the stack as an integer.
    /// Takes one byte as an operand. (The byte to be pushed.)
    bipush = 0x10,
    /// Push a short value onto the stack as an integer.
    /// Takes two bytes (byte1 and byte2) as operands.
    ///
    /// # From [jvms] :
    /// The immediate unsigned byte1 and byte2 values are assembled into an intermediate short,
    /// where the value of the short is (byte1 << 8) | byte2.
    /// The intermediate value is then sign-extended to an int
    /// value. That value is pushed onto the operand stack.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    sipush = 0x11,
    /// Push an item from the runtime constant pool onto the stack.
    /// Takes one operand byte which is the index into the runtime constant pool.
    ///
    /// # From [jvms] :
    /// ## Description:
    /// The index is an unsigned byte that must be a valid index into the
    /// run-time constant pool of the current class (§2.6). The run-time
    /// constant pool entry at index either must be a run-time constant of
    /// type int or float , or a reference to a string literal, or a symbolic
    /// reference to a class, method type, or method handle (§5.1).
    ///
    /// If the run-time constant pool entry is a run-time constant of type
    /// int or float , the numeric value of that run-time constant is pushed
    /// onto the operand stack as an int or float , respectively.
    /// Otherwise, if the run-time constant pool entry is a reference to an
    /// instance of class String representing a string literal (§5.1), then
    /// a reference to that instance, value, is pushed onto the operand
    /// stack.
    /// Otherwise, if the run-time constant pool entry is a symbolic
    /// reference to a class (§5.1), then the named class is resolved
    /// (§5.4.3.1) and a reference to the Class object representing that
    /// class, value, is pushed onto the operand stack.
    /// Otherwise, the run-time constant pool entry must be a symbolic
    /// reference to a method type or a method handle (§5.1). The method
    /// type or method handle is resolved (§5.4.3.5) and a reference
    /// to the resulting instance of java.lang.invoke.MethodType or
    /// java.lang.invoke.MethodHandle , value, is pushed onto the
    /// operand stack.
    ///
    /// ## Linking Exceptions:
    /// During resolution of a symbolic reference to a class, any of the
    /// exceptions pertaining to class resolution (§5.4.3.1) can be thrown.
    /// During resolution of a symbolic reference to a method type or
    /// method handle, any of the exception pertaining to method type or
    /// method handle resolution (§5.4.3.5) can be thrown.
    ///
    /// ## Notes:
    /// The ldc instruction can only be used to push a value of type float
    /// taken from the float value set (§2.3.2) because a constant of type
    /// float in the constant pool (§4.4.4) must be taken from the float
    /// value set.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    ldc = 0x12,
    /// Push an item from the runtime constant pool to the stack.
    /// (using wide index)
    /// Takes two byte-operands (indexbyte1 and indexbyte2) specifying the index into the
    /// runtime constant pool.
    ///
    /// # From [jvms] :
    /// The unsigned indexbyte1 and indexbyte2 are assembled into an
    /// unsigned 16-bit index into the run-time constant pool of the
    /// current class (§2.6), where the value of the index is calculated as
    /// (indexbyte1 << 8) | indexbyte2. The index must be a valid index
    /// into the run-time constant pool of the current class. The run-time
    /// constant pool entry at the index either must be a run-time constant
    /// of type int or float , or a reference to a string literal, or a
    /// symbolic reference to a class, method type, or method handle
    /// (§5.1).
    ///
    /// If the run-time constant pool entry is a run-time constant of type
    /// int or float , the numeric value of that run-time constant is pushed
    /// onto the operand stack as an int or float , respectively.
    /// Otherwise, if the run-time constant pool entry is a reference to an
    /// instance of class String representing a string literal (§5.1), then
    /// a reference to that instance, value, is pushed onto the operand
    /// stack.
    /// Otherwise, if the run-time constant pool entry is a symbolic
    /// reference to a class (§4.4.1). The named class is resolved (§5.4.3.1)
    /// and a reference to the Class object representing that class, value,
    /// is pushed onto the operand stack.
    /// Otherwise, the run-time constant pool entry must be a symbolic
    /// reference to a method type or a method handle (§5.1). The method
    /// type or method handle is resolved (§5.4.3.5) and a reference to the resulting
    /// instance of java.lang.invoke.MethodType or
    /// java.lang.invoke.MethodHandle , value, is pushed onto the
    /// operand stack.
    ///
    /// ## See [jvms] for more information.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    ldc_w = 0x13,
    /// Push a long or double from the runtime constant pool onto the stack.
    ///
    /// Takes two byte-operands. (indexbyte1 and indexbyte2)
    ///
    /// # From [jvms] :
    /// ## Description:
    /// The unsigned indexbyte1 and indexbyte2 are assembled into an
    /// unsigned 16-bit index into the run-time constant pool of the
    /// current class (§2.6), where the value of the index is calculated as
    /// (indexbyte1 << 8) | indexbyte2. The index must be a valid index
    /// into the run-time constant pool of the current class. The run-time
    /// constant pool entry at the index must be a run-time constant of
    /// type long or double (§5.1). The numeric value of that run-time
    /// constant is pushed onto the operand stack as a long or double ,
    /// respectively.
    ///
    /// ## Notes:
    /// Only a wide-index version of the ldc2_w instruction exists; there
    /// is no ldc2 instruction that pushes a long or double with a single-
    /// byte index.
    /// The ldc2_w instruction can only be used to push a value of type
    /// double taken from the double value set (§2.3.2) because a constant
    /// of type double in the constant pool (§4.4.5) must be taken from
    /// the double value set.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    lcd2_w = 0x14,
    /// Loads an integer value onto the stack from the local variable array.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    iload = 0x15,
    /// Loads a long value onto the stack from the local variable array.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    lload = 0x16,
    /// Loads a floating point value onto the stack from the local variable array.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    fload = 0x17,
    /// Loads a double precision value onto the stack from the local variable array.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    dload = 0x18,
    /// Loads an object reference onto the stack from the local variable array.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    aload = 0x19,
    /// Loads an integer value onto the stack from the index 0 in the local variable array.
    iload_0 = 0x1a,
    /// Loads an integer value onto the stack from the index 1 in the local variable array.
    iload_1 = 0x1b,
    /// Loads an integer value onto the stack from the index 2 in the local variable array.
    iload_2 = 0x1c,
    /// Loads an integer value onto the stack from the index 3 in the local variable array.
    iload_3 = 0x1d,
    /// Loads a long value onto the stack from the index 0 in the local variable array.
    lload_0 = 0x1e,
    /// Loads a long value onto the stack from the index 1 in the local variable array.
    lload_1 = 0x1f,
    /// Loads a long value onto the stack from the index 2 in the local variable array.
    lload_2 = 0x20,
    /// Loads a long value onto the stack from the index 3 in the local variable array.
    lload_3 = 0x21,
    /// Loads a floating point value onto the stack from the index 0 in the local variable array.
    fload_0 = 0x22,
    /// Loads a floating point value onto the stack from the index 1 in the local variable array.
    fload_1 = 0x23,
    /// Loads a floating point value onto the stack from the index 2 in the local variable array.
    fload_2 = 0x24,
    /// Loads a floating point value onto the stack from the index 3 in the local variable array.
    flaod_3 = 0x25,
    /// Loads a double precision value onto the stack from the index 0 in the local variable array.
    dload_0 = 0x26,
    /// Loads a double precision value onto the stack from the index 1 in the local variable array.
    dload_1 = 0x27,
    /// Loads a double precision value onto the stack from the index 2 in the local variable array.
    dload_2 = 0x28,
    /// Loads a double precision value onto the stack from the index 3 in the local variable array.
    dload_3 = 0x29,
    /// Loads an object reference onto the stack from the index 0 in the local variable array.
    aload_0 = 0x2a,
    /// Loads an object reference onto the stack from the index 1 in the local variable array.
    aload_1 = 0x2b,
    /// Loads an object reference onto the stack from the index 2 in the local variable array.
    aload_2 = 0x2c,
    /// Loads an object reference onto the stack from the index 3 in the local variable array.
    aload_3 = 0x2d,
    /// Loads an integer from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of integers.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    ///
    ialaod = 0x2e,
    /// Loads a long from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of longs.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    ///
    lalaod = 0x2f,
    /// Loads a float from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of floats.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    falaod = 0x30,
    /// Loads a double from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of doubles.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    dalaod = 0x31,
    /// Loads a reference from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of references.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    aalaod = 0x32,
    /// Loads a byte or boolean value from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of bytes or booleans.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    ///
    /// # From [jvms]:
    /// ## Notes:
    /// The baload instruction is used to load values from both byte and
    /// boolean arrays. In Oracle's Java Virtual Machine implementation,
    /// boolean arrays - that is, arrays of type T_BOOLEAN (§2.2,
    /// §[`newarray`]) - are implemented as arrays of 8-bit values. Other
    /// implementations may implement packed boolean arrays; the
    /// baload instruction of such implementations must be used to access
    /// those arrays.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    /// [`newarray`]: ./enum.OpCode.html#variant.newarray
    balaod = 0x33,
    /// Loads a character from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of characters.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    calaod = 0x34,
    /// Loads a short from an array onto the stack.
    ///
    /// Pops two values off the stack:
    /// arrayref, index
    /// ->
    /// value
    ///
    /// arrayref and index are removed from the stack, and the value at the index in the referenced
    /// array is pushed onto the stack.
    ///
    /// arrayref must refer to an array of shorts.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// If arrayref refers to null, then a NullPointerException is thrown.
    /// If index is out of the bounds of the referenced array, an ArrayIndexOutOfBoundsException is
    /// thrown.
    saload = 0x35,
    /// Stores an integer from the stack into a local variable.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (the value taken off the stack is stored into the local variable at the index)
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    istore = 0x36,
    /// Stores a long from the stack into a local variable.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (the value taken off the stack is stored into the local variable at the index)
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    lstore = 0x37,
    /// Stores a float from the stack into a local variable.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (the value taken off the stack is stored into the local variable at the index)
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    fstore = 0x38,
    /// Stores a double from the stack into a local variable.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (the value taken off the stack is stored into the local variable at the index)
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    dstore = 0x39,
    /// Stores a reference from the stack into a local variable.
    ///
    /// Takes one byte-operand (the index into the local variable array).
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (the value taken off the stack is stored into the local variable at the index)
    ///
    /// May take two bytes when used in conjunction with the [wide] opcode.
    /// (see the [wide] opcode for more information)
    ///
    /// [wide]: ./enum.OpCode.html#variant.wide
    astore = 0x3a,
    /// Stores an integer from the stack into local variable 0.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [istore] #0)
    ///
    /// [istore]: ./enum.OpCode.html#variant.istore
    istore_0 = 0x3b,
    /// Stores an integer from the stack into local variable 1.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [istore] #1)
    ///
    /// [istore]: ./enum.OpCode.html#variant.istore
    istore_1 = 0x3c,
    /// Stores an integer from the stack into local variable 2.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [istore] #2)
    ///
    /// [istore]: ./enum.OpCode.html#variant.istore
    istore_2 = 0x3d,
    /// Stores an integer from the stack into local variable 3.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [istore] #3)
    ///
    /// [istore]: ./enum.OpCode.html#variant.istore
    istore_3 = 0x3e,
    /// Stores a long from the stack into local variable 0.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [lstore] #0)
    ///
    /// [lstore]: ./enum.OpCode.html#variant.lstore
    lstore_0 = 0x3f,
    /// Stores a long from the stack into local variable 1.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [lstore] #1)
    ///
    /// [lstore]: ./enum.OpCode.html#variant.lstore
    lstore_1 = 0x40,
    /// Stores a long from the stack into local variable 2.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [lstore] #2)
    ///
    /// [lstore]: ./enum.OpCode.html#variant.lstore
    lstore_2 = 0x41,
    /// Stores a long from the stack into local variable 3.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [lstore] #3)
    ///
    /// [lstore]: ./enum.OpCode.html#variant.lstore
    lstore_3 = 0x42,
    /// Stores a float from the stack into local variable 0.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [fstore] #0)
    ///
    /// [fstore]: ./enum.OpCode.html#variant.fstore
    fstore_0 = 0x43,
    /// Stores a float from the stack into local variable 1.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [fstore] #1)
    ///
    /// [fstore]: ./enum.OpCode.html#variant.fstore
    fstore_1 = 0x44,
    /// Stores a float from the stack into local variable 2.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [fstore] #2)
    ///
    /// [fstore]: ./enum.OpCode.html#variant.fstore
    fstore_2 = 0x45,
    /// Stores a float from the stack into local variable 3.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [fstore] #3)
    ///
    /// [fstore]: ./enum.OpCode.html#variant.fstore
    fstore_3 = 0x46,
    /// Stores a double from the stack into local variable 0.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [dstore] #0)
    ///
    /// [dstore]: ./enum.OpCode.html#variant.dstore
    dstore_0 = 0x47,
    /// Stores a double from the stack into local variable 1.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [dstore] #1)
    ///
    /// [dstore]: ./enum.OpCode.html#variant.dstore
    dstore_1 = 0x48,
    /// Stores a double from the stack into local variable 2.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [dstore] #2)
    ///
    /// [dstore]: ./enum.OpCode.html#variant.dstore
    dstore_2 = 0x49,
    /// Stores a double from the stack into local variable 3.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [dstore] #3)
    ///
    /// [dstore]: ./enum.OpCode.html#variant.dstore
    dstore_3 = 0x4a,
    /// Stores a reference from the stack into local variable 0.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [astore] #0)
    ///
    /// [astore]: ./enum.OpCode.html#variant.astore
    astore_0 = 0x4b,
    /// Stores a reference from the stack into local variable 1.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [astore] #1)
    ///
    /// [astore]: ./enum.OpCode.html#variant.astore
    astore_1 = 0x4c,
    /// Stores a reference from the stack into local variable 2.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [astore] #2)
    ///
    /// [astore]: ./enum.OpCode.html#variant.astore
    astore_2 = 0x4d,
    /// Stores a reference from the stack into local variable 3.
    ///
    /// Pops one value off the stack:
    /// value
    /// ->
    ///
    /// (similar to [astore] #3)
    ///
    /// [astore]: ./enum.OpCode.html#variant.astore
    astore_3 = 0x4e,
    /// Stores an integer value into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of integers.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Runtime Exceptions:
    /// If arrayref is null, iastore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the iastore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    iastore = 0x4f,
    /// Stores a long value into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of longs.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Runtime Exceptions:
    /// If arrayref is null, lastore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the lastore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    lastore = 0x50,
    /// Stores a floating point value into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of floats.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Runtime Exceptions:
    /// If arrayref is null, fastore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the fastore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    fastore = 0x51,
    /// Stores a double value into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of doubles.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Runtime Exceptions:
    /// If arrayref is null, dastore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the dastore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    dastore = 0x52,
    /// Stores a reference into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of references.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Description:
    /// The arrayref must be of type reference and must refer to an array
    /// whose components are of type reference. The index must be of
    /// type int, and value must be of type reference. The arrayref,
    /// index, and value are popped from the operand stack.
    /// If value is null, then value is stored as the component of the array
    /// at index.
    /// Otherwise, value is non-null. If the type of value is assignment
    /// compatible with the type of the components of the array referenced
    /// by arrayref, then value is stored as the component of the array at
    /// index.
    /// The following rules are used to determine whether a value that
    /// is not null is assignment compatible with the array component
    /// type. If S is the type of the object referred to by value, and T is the
    /// reference type of the array components, then aastore determines
    /// whether assignment is compatible as follows:
    /// • If S is a class type, then:
    /// – If T is a class type, then S must be the same class as T , or S
    /// must be a subclass of T ;
    /// – If T is an interface type, then S must implement interface T .
    /// • If S is an array type SC[] , that is, an array of components of type
    /// SC , then:
    /// – If T is a class type, then T must be Object .
    /// – If T is an interface type, then T must be one of the interfaces
    /// implemented by arrays (JLS §4.10.3).
    /// – If T is an array type TC[] , that is, an array of components of
    /// type TC , then one of the following must be true:
    /// › TC and SC are the same primitive type.
    /// › TC and SC are reference types, and type SC is assignable to
    /// TC by these run-time rules.
    ///
    /// ## Runtime Exceptions:
    /// If arrayref is null, aastore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the aastore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    /// Otherwise, if arrayref is not null and the actual type of
    /// the non-null value is not assignment compatible with the
    /// actual type of the components of the array, aastore throws an
    /// ArrayStoreException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    aastore = 0x53,
    /// Stores a byte or boolean value into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of bytes or booleans.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Description:
    /// If the arrayref refers to an array whose components are of type
    /// boolean, then the int value is narrowed by taking the bitwise
    /// AND of value and 1; the result is stored as the component of the
    /// array indexed by index.
    ///
    /// ## Runtime Exceptions:
    /// If arrayref is null, bastore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the bastore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    bastore = 0x54,
    /// Stores a character into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of characters.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Runtime Exceptions:
    /// If arrayref is null, castore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the castore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    castore = 0x55,
    /// Stores a short value into an array.
    ///
    /// Pops 3 values off the stack:
    /// arrayref, index, value
    /// ->
    ///
    ///
    /// arrayref must refer to an array of shorts.
    /// index must be an integer value.
    /// If either of these preconditions are not met, the behavior of the jvm is undefined.
    ///
    /// # From [jvms]:
    /// ## Runtime Exceptions:
    /// If arrayref is null, sastore throws a NullPointerException.
    /// Otherwise, if index is not within the bounds of the array
    /// referenced by arrayref, the sastore instruction throws an
    /// ArrayIndexOutOfBoundsException.
    ///
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    sastore = 0x56,
    /// Pop a value off of the stack discard it.
    ///
    /// [`pop`] cannot be used if the top value on the stack is a long or a double.
    /// If it is, the behavior of the JVM is undefined.
    ///
    /// [`pop`]: ./enum.OpCode.html#variant.pop
    pop = 0x57,
    /// Pop two values off the top of the stack and discard them.
    ///
    /// (One if the value at the top of the stack is a double or a long)
    pop2 = 0x58,
    /// Duplicates the top value of the stack.
    ///
    /// value
    /// ->
    /// value, value
    ///
    /// [`dup`] cannot be used if the top value on the stack is a long or a double.
    /// If it is, the behavior of the JVM is undefined.
    ///
    /// [`dup`]: ./enum.OpCode.html#variant.dup
    dup = 0x59,
    /// Insert a copy of the top value of the stack two values down from the top.
    ///
    /// value1, value2
    /// ->
    /// value2, value1, value2
    ///
    /// [`dup_x1`] cannot be used if the top 2 values of the stack doubles or longs.
    /// If they are, the behavior of the JVM is undefined.
    ///
    /// [`dup_x1`]: ./enum.OpCode.html#variant.dup_x1
    dup_x1 = 0x5a,
    /// Insert a copy of the top value of the stack two (or three) values down from the top.
    ///
    /// (if value1 & value2 are individual values, neither of which is a long or double)
    /// value1, value2, value3
    /// ->
    /// value3, value1, value2, value3
    ///
    /// (if value1 is a long or double)
    /// value1, value2
    /// ->
    /// value2, value1, value2
    ///
    /// [`dup_x2`] cannot be used if the top value of the stack is a double or long.
    /// If it is, the behavior of the JVM is undefined.
    ///
    /// [`dup_x2`]: ./enum.OpCode.html#variant.dup_x1
    dup_x2 = 0x5b,
    /// Duplicates the top 2 values of the stack.
    /// (One value if the top value on the stack is a double long.)
    ///
    /// {value1, value2}
    /// ->
    /// {value1, value2}, {value1, value2}
    dup2 = 0x5c,
    dup2_x1 = 0x5d,
    dup2_x2 = 0x5e,
    /// Swaps the top value of the stack.
    ///
    /// value1, value2
    /// ->
    /// value2, value1
    ///
    /// [`swap`] cannot be used if the top 2 values of the stack doubles or longs.
    /// If they are is, the behavior of the JVM is undefined.
    ///
    /// [`swap`]: ./enum.OpCode.html#variant.swap
    swap = 0x5f,
    /// Add two integers at the top of the stack, and put the result back on.
    ///
    /// value1, value2
    /// ->
    /// result
    ///
    /// value1 and value2 both have to be integers.
    /// If they are not, the behavior of the JVM is undefined.
    iadd = 0x60,
    // todo: docs
    ladd = 0x61,
    fadd = 0x62,
    dadd = 0x63,
    isub = 0x64,
    lsub = 0x65,
    fsub = 0x66,
    dsub = 0x67,
    imul = 0x68,
    lmul = 0x69,
    fmul = 0x6a,
    dmul = 0x6b,
    idiv = 0x6c,
    ldiv = 0x6d,
    fdiv = 0x6e,
    ddiv = 0x6f,
    irem = 0x70,
    lrem = 0x71,
    frem = 0x72,
    drem = 0x73,
    ineg = 0x74,
    lneg = 0x75,
    fneg = 0x76,
    dneg = 0x77,
    ishl = 0x78,
    lshl = 0x79,
    ishr = 0x7a,
    lshr = 0x7b,
    iushr = 0x7c,
    lushr = 0x7d,
    iand = 0x7e,
    land = 0x7f,
    ior = 0x80,
    lor = 0x81,
    ixor = 0x82,
    lxor = 0x83,
    iinc = 0x84,
    /// Converts an integer to a long.
    /// Pops the integer value off the stack and puts it back on as a long.
    ///
    /// integer -> long
    i2l = 0x85,
    /// Converts an integer to a float.
    /// Pops the integer value off the stack and puts it back on as a float.
    ///
    /// integer -> float
    i2f = 0x86,
    /// Converts an integer to a double.
    /// Pops the integer value off the stack and puts it back on as a double.
    ///
    /// integer -> double
    i2d = 0x87,
    /// Converts a long to an integer.
    /// Pops the long value off the stack and puts it back on as an integer.
    ///
    /// long -> integer
    l2i = 0x88,
    /// Converts a long to a float.
    /// Pops the long value off the stack and puts it back on as a float.
    ///
    /// long -> float
    l2f = 0x89,
    /// Converts a long to a double.
    /// Pops the long value off the stack and puts it back on as a double.
    ///
    /// long -> double
    l2d = 0x8a,
    /// Converts a float to an integer.
    /// Pops the float value off the stack and puts it back on as an integer.
    ///
    /// float -> integer
    f2i = 0x8b,
    /// Converts a float to a long.
    /// Pops the float value off the stack and puts it back on as a long.
    ///
    /// float -> long
    f2l = 0x8c,
    /// Converts a float to a double.
    /// Pops the float value off the stack and puts it back on as a double .
    ///
    /// float -> double
    f2d = 0x8d,
    /// Converts a double to an integer.
    /// Pops the double value off the stack and puts it back on as an integer.
    ///
    /// double -> integer
    d2i = 0x8e,
    /// Converts a double to a long.
    /// Pops the double value off the stack and puts it back on as a long.
    ///
    /// double -> long
    d2l = 0x8f,
    /// Converts a double to a float.
    /// Pops the double value off the stack and puts it back on as a float.
    ///
    /// double -> float
    d2f = 0x90,
    /// Converts an integer to a byte.
    /// Pops the integer value off the stack and puts it back on as a byte.
    ///
    /// integer -> byte
    i2b = 0x91,
    /// Converts an integer to a character.
    /// Pops the integer value off the stack and puts it back on as a character.
    ///
    /// integer -> character
    i2c = 0x92,
    /// Converts an integer to a short.
    /// Pops the integer value off the stack and puts it back on as a short.
    ///
    /// integer -> short
    i2s = 0x93,
    // todo: docs
    lcmp = 0x94,
    fcmpl = 0x95,
    fcmpg = 0x96,
    dcmpl = 0x97,
    dcmpg = 0x98,
    ifeq = 0x99,
    ifne = 0x9a,
    iflt = 0x9b,
    ifge = 0x9c,
    ifgt = 0x9d,
    ifle = 0x9e,
    if_icmpeq = 0x9f,
    if_icmpne = 0xa0,
    if_icmplt = 0xa1,
    if_icmpge = 0xa2,
    if_icmpgt = 0xa3,
    if_icmple = 0xa4,
    if_acmpeq = 0xa5,
    if_acmpne = 0xa6,
    goto = 0xa7,
    jsr = 0xa8,
    ret = 0xa9,
    tableswitch = 0xaa,
    lookupswitch = 0xab,
    ireturn = 0xac,
    lreturn = 0xad,
    freturn = 0xae,
    dreturn = 0xaf,
    areturn = 0xb0,
    _return = 0xb1,
    getstatic = 0xb2,
    putstatic = 0xb3,
    getfield = 0xb4,
    putfield = 0xb5,
    invokevirtual = 0xb6,
    invokespecial = 0xb7,
    invokestatic = 0xb8,
    invokeinterface = 0xb9,
    invokedynamic = 0xba,
    new = 0xbb,
    newarray = 0xbc,
    anewarray = 0xbd,
    arraylength = 0xbe,
    athrow = 0xbf,
    checkcast = 0xc0,
    instanceof = 0xc1,
    monitorenter = 0xc2,
    monitorexit = 0xc3,
    wide = 0xc4,
    multianewarray = 0xc5,
    ifnull = 0xc6,
    ifnonnull = 0xc7,
    goto_w = 0xc8,
    jsr_w = 0xc9,


    /// Breakpoint opcode to be used only internally by debuggers.
    /// Will not be in any valid class files.
    breakpoint = 0xca,
    /// Undefined opcode 0xCB.
    undefined_0xCB = 0xcb,
    /// Undefined opcode 0xCC.
    undefined_0xCC = 0xcc,
    /// Undefined opcode 0xCD.
    undefined_0xCD = 0xcd,
    /// Undefined opcode 0xCE.
    undefined_0xCE = 0xce,
    /// Undefined opcode 0xCF.
    undefined_0xCF = 0xcf,
    /// Undefined opcode 0xD1.
    undefined_0xD1 = 0xd1,
    /// Undefined opcode 0xD2.
    undefined_0xD2 = 0xd2,
    /// Undefined opcode 0xD3.
    undefined_0xD3 = 0xd3,
    /// Undefined opcode 0xD4.
    undefined_0xD4 = 0xd4,
    /// Undefined opcode 0xD5.
    undefined_0xD5 = 0xd5,
    /// Undefined opcode 0xD6.
    undefined_0xD6 = 0xd6,
    /// Undefined opcode 0xD7.
    undefined_0xD7 = 0xd7,
    /// Undefined opcode 0xD8.
    undefined_0xD8 = 0xd8,
    /// Undefined opcode 0xD9.
    undefined_0xD9 = 0xd9,
    /// Undefined opcode 0xDA.
    undefined_0xDA = 0xda,
    /// Undefined opcode 0xDB.
    undefined_0xDB = 0xdb,
    /// Undefined opcode 0xDC.
    undefined_0xDC = 0xdc,
    /// Undefined opcode 0xDD.
    undefined_0xDD = 0xdd,
    /// Undefined opcode 0xDE.
    undefined_0xDE = 0xde,
    /// Undefined opcode 0xDF.
    undefined_0xDF = 0xdf,
    /// Undefined opcode 0xF1.
    undefined_0xF1 = 0xf1,
    /// Undefined opcode 0xF2.
    undefined_0xF2 = 0xf2,
    /// Undefined opcode 0xF3.
    undefined_0xF3 = 0xf3,
    /// Undefined opcode 0xF4.
    undefined_0xF4 = 0xf4,
    /// Undefined opcode 0xF5.
    undefined_0xF5 = 0xf5,
    /// Undefined opcode 0xF6.
    undefined_0xF6 = 0xf6,
    /// Undefined opcode 0xF7.
    undefined_0xF7 = 0xf7,
    /// Undefined opcode 0xF8.
    undefined_0xF8 = 0xf8,
    /// Undefined opcode 0xF9.
    undefined_0xF9 = 0xf9,
    /// Undefined opcode 0xFA.
    undefined_0xFA = 0xfa,
    /// Undefined opcode 0xFB.
    undefined_0xFB = 0xfb,
    /// Undefined opcode 0xFC.
    undefined_0xFC = 0xfc,
    /// Undefined opcode 0xFD.
    undefined_0xFD = 0xfd,
    /// Implementation dependent opcode 1. Reserved to be used internally by tools like debuggers
    /// or JIT implementations. Will not show up in class files.
    /// See [jvms] for more information.
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    impdep1 = 0xfe,
    /// Implementation dependent opcode 2. Reserved to be used internally by tools like debuggers
    /// or JIT implementations. Will not show up in class files.
    /// See [jvms] for more information.
    /// [jvms]: https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf
    impdep2 = 0xff,
}


impl OpCode {
    /// Converts a byte (u8) to a jvm opcode.
    /// # Note:
    /// This function uses the **incredibly unsafe** [`mem::transmute`] under the hood.
    /// While it should, by design, never fail, it is the only use of unsafe in wright, and should
    /// be taken seriously as such.
    /// [`mem::transmute`]: https://doc.rust-lang.org/std/mem/fn.transmute.html
    pub fn from_byte(byte: u8) -> OpCode { unsafe { ::std::mem::transmute::<u8, OpCode>(byte) } }
    /// Convert an opcode to a byte (u8).
    pub fn to_byte(self) -> u8 { self as u8 }
}
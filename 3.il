.assembly extern System.Runtime {}
.assembly extern System.Console {}
.assembly extern System.IO.FileSystem {}
.assembly extern System.Private.CoreLib {}
.assembly AoC.Task3 {}
.module AoC.Task3

// Indentation indicates stack depth (before the instruction)

.class public Program
{
    .method static public hidebysig void Main() cil managed
    {
        .entrypoint

        .locals (
            [0] class [System.Runtime]System.IO.StreamReader,
            [1] string, // firstline
            [2] int32, // width
            [3] string, // restlines
            [4] int32, // height
            [5] char[] // map
        )

        .maxstack 5

        ldstr "3.input"
         call class [System.Runtime]System.IO.StreamReader [System.IO.FileSystem]System.IO.File::OpenText(string)
         dup
          stloc.0
         brfalse.s L_END

        // Read first line
        ldloc.0
         callvirt instance string [System.Runtime]System.IO.StreamReader::ReadLine()
         dup
          stloc.1
         call instance int32 [System.Runtime]System.String::get_Length()
         stloc.2

        // Read rest of lines
        ldloc.0
         callvirt instance string [System.Runtime]System.IO.StreamReader::ReadToEnd()
         dup
          stloc.3
         call instance int32 [System.Runtime]System.String::get_Length()
        // Divide by (width + 1) to get the number of lines - 1
         ldloc.2
          ldc.i4.1
           add
          div.un
         ldc.i4.1
          add
         stloc 4 // Store the height

        // Make the char array
        ldloc.2
         ldloc 4
          mul.ovf.un
         newarr char
         stloc 5

        // Copy first line into the array
        ldloc.1  // First line
         ldc.i4.0 // source index
          ldloc 5  // buffer
           ldc.i4.0 // dest index
            ldloc.2  // count
             call instance void [System.Runtime]System.String::CopyTo(int32, char[], int32, int32)

        // Copy the rest into the buffer
        ldloc 5
         ldloc.3
          ldloc.2
           ldloc 4
        // (height - 1) lines
            ldc.i4.1
             sub.ovf.un
            call void Program::HelperCopyChunksArray(char[], string, int32, int32)

        // Finally! Solve the task
        ldloc 5
         ldloc.2
          ldloc 4
           call void Program::SolveTask2(char[], int32, int32)

        ldloc.0
         callvirt instance void [System.Runtime]System.IDisposable::Dispose()

        L_END:
        ret
    }

    .method static private hidebysig
        void HelperCopyChunksArray(char[] buffer, string str, int32 width, int32 count) cil managed
    {
        .locals (
            [0] int32, // stride = width + 1
            [1] int32 // loop counter
        )

        .maxstack 5

        ldarg.2
        ldc.i4.1
        add
        stloc.0

        ldc.i4.0
        stloc.1

        L_LOOP_BEGIN:
        ldarg.1 // The source string

        ldloc.1
        ldloc.0
        mul.ovf.un // Source index

        ldarg.0 // Buffer

        ldloc.1
        ldc.i4.1
        add
        ldarg.2
        mul.ovf.un // Destination index = (i + 1) * width

        ldarg.2 // Count
        call instance void [System.Runtime]System.String::CopyTo(int32, char[], int32, int32)

        // Increment loop counter
        ldloc.1
        ldc.i4.1
        add
        dup
        stloc.1

        // Check if reached count
        ldarg.3
        ceq
        brfalse.s L_LOOP_BEGIN

        // All done :)
        ret
    }

    .method static private hidebysig
        void SolveTask1(char[] map, int32 width, int32 height) cil managed
    {
        .locals (
            [0] int32
        )

        .maxstack 5
        ldarg.0
         ldarg.1
          ldarg.2
           ldc.i4.3
            ldc.i4.1
             call int32 Program::ExecuteSlope(char[], int32, int32, int32, int32)
         stloc.0

        // Print the count and done.
        ldstr "Count: {0}"
         ldloc.0
         //box [System.Runtime]System.Int32
         box int32
          call void [System.Console]System.Console::WriteLine(string, object)

        ret
    }

    .method static private hidebysig
        void SolveTask2(char[] map, int32 width, int32 height) cil managed
    {
        .locals (
            [0] int32
        )

        .maxstack 5
        // First slope 1:1
        ldarg.0
         ldarg.1
          ldarg.2
           ldc.i4.1
            ldc.i4.1
             call int32 Program::ExecuteSlope(char[], int32, int32, int32, int32)
         stloc.0
        
        // Second slope 3:1
        ldarg.0
         ldarg.1
          ldarg.2
           ldc.i4.3
            ldc.i4.1
             call int32 Program::ExecuteSlope(char[], int32, int32, int32, int32)
         ldloc.0
          mul.ovf.un
         stloc.0
        
        // Third slope 5:1
        ldarg.0
         ldarg.1
          ldarg.2
           ldc.i4.5
            ldc.i4.1
             call int32 Program::ExecuteSlope(char[], int32, int32, int32, int32)
         ldloc.0
          mul.ovf.un
         stloc.0
        
        // Fourth slope 7:1
        ldarg.0
         ldarg.1
          ldarg.2
           ldc.i4.7
            ldc.i4.1
             call int32 Program::ExecuteSlope(char[], int32, int32, int32, int32)
         ldloc.0
          mul.ovf.un
         stloc.0
        
        // Fifth slope 1:2
        ldarg.0
         ldarg.1
          ldarg.2
           ldc.i4.1
            ldc.i4.2
             call int32 Program::ExecuteSlope(char[], int32, int32, int32, int32)
         ldloc.0
          mul.ovf.un
         stloc.0

        // Print the count and done.
        ldstr "Count: {0}"
         ldloc.0
         //box [System.Runtime]System.Int32
         box int32
          call void [System.Console]System.Console::WriteLine(string, object)

        ret
    }

    .method static private hidebysig
        int32 ExecuteSlope(char[] map, int32 width, int32 height, int32 dirX, int32 dirY) cil managed
    {
        .locals (
            [0] int32, // current x
            [1] int32, // current y
            [2] int32  // trees encountered
        )

        .maxstack 3

        ldc.i4.0
         dup
          stloc.0
         dup
          stloc.1
         stloc.2
        
        L_LOOP_BEGIN:
        // Calculate index = x + y*width
        ldarg.0
         ldloc.1
          ldarg.1
           mul.ovf.un
          ldloc.0
           add
          ldelem.u2
         conv.ovf.i4.un
        
        // Check if tree
         ldc.i4 0x23 // '#'
          ceq
        // Add to the tree counter
         ldloc.2
          add
         stloc.2

        // Increment coordinates
        ldloc.0
         ldarg.3
          add
         ldarg.1
          rem.un // Take reminder
         stloc.0

        ldloc.1
         ldarg 4
          add
         dup
          stloc.1
        // Check if out the end
         ldarg.2
          blt.un.s L_LOOP_BEGIN

        ldloc.2
        ret
    }
}

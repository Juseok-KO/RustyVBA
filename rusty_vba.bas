Private Declare PtrSafe Function WideCharToMultiByte Lib "kernel32" ( _
        ByVal CodePage As Long, ByVal dwFlags As Long, _
        ByVal lpWideCharStr As LongPtr, ByVal cchWideChar As Long, _
        ByVal lpMultiByteStr As LongPtr, ByVal cbMultiByte As Long, _
        ByVal lpDefaultChar As LongPtr, ByVal lpUsedDefaultChar As LongPtr) As Long

Private Declare PtrSafe Function MultiByteToWideChar Lib "kernel32" ( _
        ByVal CodePage As Long, ByVal dwFlags As Long, _
        ByVal lpMultiByteStr As LongPtr, ByVal cbMultiByte As Long, _
        ByVal lpWideCharStr As LongPtr, ByVal cchWideChar As Long) As Long

Const CP_UTF8 As Long = 65001

Function VBAStringToCString(ByVal s As String) As LongPtr
    Dim bytesNeeded As Long
    Dim buffer() As Byte
    
    ' First call to get required buffer size
    bytesNeeded = WideCharToMultiByte(CP_UTF8, 0, StrPtr(s), -1, 0, 0, 0, 0)
    
    If bytesNeeded > 0 Then
        ReDim buffer(0 To bytesNeeded - 1) As Byte
        WideCharToMultiByte CP_UTF8, 0, StrPtr(s), -1, VarPtr(buffer(0)), bytesNeeded, 0, 0
        VBAStringToCString = VarPtr(buffer(0)) ' pointer to C-style string
    End If
End Function

Function CStringToVBAString(ByVal cStrPtr As LongPtr) As String
    Dim charsNeeded As Long
    Dim result As String
    
    ' First, find length of UTF-8 string (null-terminated)
    charsNeeded = MultiByteToWideChar(CP_UTF8, 0, cStrPtr, -1, 0, 0)
    If charsNeeded > 0 Then
        result = String$(charsNeeded - 1, vbNullChar) ' allocate space (minus null terminator)
        MultiByteToWideChar CP_UTF8, 0, cStrPtr, -1, StrPtr(result), charsNeeded - 1
        CStringToVBAString = result
    End If
End Function
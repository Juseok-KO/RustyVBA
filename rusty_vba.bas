Const CP_UTF8 As Long = 65001
Const TYPE_NULL_PTR As LongLong = -1
Const TYPE_NONE As LongLong = 0
Const TYPE_I8 As LongLong = 1
Const TYPE_I16 As LongLong = 2
Const TYPE_I32 As LongLong = 3
Const TYPE_I64 As LongLong = 4
Const TYPE_F32 As LongLong = 5
Const TYPE_F64 As LongLong = 6
Const TYPE_BOOL As LongLong = 7
Const TYPE_CSTRING As LongLong = 8
Const TYPE_ARRAY As LongLong = 9
Const RUST_TRUE As Byte = 1
Const RUST_FALSE As Byte = 0


Private Declare PtrSafe Function drop_data Lib "{}" ( _
    ByVal ptr_data As LongPtr) As Boolean

Private Declare PtrSafe Function get_type Lib "{}" ( _
    ByVal ptr_data As LongPtr) As LongLong
    
Private Declare PtrSafe Function get_i8 Lib "{}" ( _
    ByVal ptr_data As LongPtr) As Byte
    
Private Declare PtrSafe Function get_i16 Lib "{}" ( _
    ByVal ptr_data As LongPtr) As Integer
    
Private Declare PtrSafe Function get_i32 Lib "{}" ( _
    ByVal ptr_data As LongPtr) As Long

Private Declare PtrSafe Function get_i64 Lib "{}" ( _
    ByVal ptr_data As LongPtr) As LongLong

Private Declare PtrSafe Function get_f32 Lib "{}" ( _
    ByVal ptr_data As LongPtr) As Single
    
Private Declare PtrSafe Function get_f64 Lib "{}" ( _
    ByVal ptr_data As LongPtr) As Double

Private Declare PtrSafe Function get_bool Lib "{}" ( _
    ByVal ptr_data As LongPtr) As Byte

Private Declare PtrSafe Function get_ptr_str Lib "{}" ( _
    ByVal PtrRustStr As LongPtr _
) As LongPtr

Private Declare PtrSafe Function init_array Lib "{}" ( _
    ByVal row As LongLong, _
    ByVal col As LongLong _
) As LongPtr

Private Declare PtrSafe Function arr_num_rows Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_num_cols Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i8 Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Byte, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i16 Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Integer, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i32 Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Long, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i64 Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As LongLong, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_f32 Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Single, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_f64 Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Double, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_bool Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Byte, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function get_elem_ptr Lib "{}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function MultiByteToWideChar Lib "kernel32" ( _
        ByVal CodePage As Long, ByVal dwFlags As Long, _
        ByVal lpMultiByteStr As LongPtr, ByVal cbMultiByte As Long, _
        ByVal lpWideCharStr As LongPtr, ByVal cchWideChar As Long) As Long


Private Function CStringToVBAString(ByVal cStrPtr As LongPtr) As String
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

Private Function CreateArrayFromPointer(ByVal ptr_arr As LongPtr) As Variant
    
    Dim result() As Variant
    Dim ptr_num As LongPtr
    Dim num_row As Long
    Dim num_col As Long
    
    Dim error As Byte
    Dim ptr_err As LongPtr
    error = 0
    ptr_err = VarPtr(error)
    
    ptr_num = arr_num_rows(ptr_arr, ptr_err)
    
    If error = RUST_FALSE Then
        CreateArrayFromPointer = "Failed to get the number of rows"
        Exit Function
    End If
    
    num_row = get_i32(ptr_num)
    ptr_num = drop_data(ptr_num)
    
    ptr_num = arr_num_cols(ptr_arr, ptr_err)
    
    If error = RUST_FALSE Then
        CreateArrayFromPointer = "Failed to get the number of cols"
        Exit Function
    End If
    
    num_col = get_i32(ptr_num)
    ptr_num = drop_data(ptr_num)
    
    ReDim result(num_row - 1, num_col - 1)
    
    Dim cur_row As Long
    Dim cur_col As Long
    Dim ptr_val As LongPtr
    Dim val As Variant
    
    For cur_row = 0 To num_row - 1
    
        For cur_col = 0 To num_col - 1
            error = 0
            
            ptr_val = get_elem_ptr(ptr_arr, cur_row, cur_col, ptr_err)
            
            If error = RUST_FALSE Then
                MsgBox "Error occured"
                
                CreateArrayFromPointer = "Failed to get element at " & cur_row & ", " & cur_col
                Exit Function
            End If
            
            val = ReadPtrData(ptr_val)
            result(cur_row, cur_col) = val
        
        Next cur_col
    
    Next cur_row
    
    CreateArrayFromPointer = result

End Function

Private Function ReadPtrData(ByVal ptr_data As LongPtr) As Variant
    
    Dim dtype As LongLong
    Dim value As Variant
    Dim ptr_str As LongPtr
    Dim rust_bool As Byte
    dtype = get_type(ptr_data)
    
    MsgBox "dtype: " & dtype
    
    If dtype = TYPE_NULL_PTR Then
        value = "!Null Pointer detected"
    
    ElseIf dtype = TYPE_NONE Then
        value = Null
    
    ElseIf dtype = TYPE_I8 Then
        value = get_i8(ptr_data)
    
    ElseIf dtype = TYPE_I16 Then
        value = get_i16(ptr_data)
    
    ElseIf dtype = TYPE_I32 Then
        value = get_i32(ptr_data)
    
    ElseIf dtype = TYPE_I64 Then
        value = get_i64(ptr_data)
    
    ElseIf dtype = TYPE_F32 Then
    
        value = get_f32(ptr_data)
    
    ElseIf dtype = TYPE_F64 Then
    
        value = get_f64(ptr_data)
        
    ElseIf dtype = TYPE_CSTRING Then
    
        ptr_str = get_ptr_str(ptr_data)
        value = CStringToVBAString(ptr_str)
    
    ElseIf dtype = TYPE_BOOL Then
    
        rust_bool = get_bool(ptr_data)
        
        If rust_bool = RUST_TRUE Then
            value = True
        Else
            value = False
        End If
        
    ElseIf dtype = TYPE_ARR Then
        value = CreateArrayFromPointer(ptr_data)
        
    End If
    
    ReadPtrData = value

End Function
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

Const DLL_DIR_ROOT As String = "{DLL_ROOT}"

Private Declare PtrSafe Function drop_data Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As Byte

Private Declare PtrSafe Function get_type Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As LongLong
    
Private Declare PtrSafe Function get_i8 Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As Byte
    
Private Declare PtrSafe Function get_i16 Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As Integer
    
Private Declare PtrSafe Function get_i32 Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As Long

Private Declare PtrSafe Function get_i64 Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As LongLong

Private Declare PtrSafe Function get_f32 Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As Single
    
Private Declare PtrSafe Function get_f64 Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As Double

Private Declare PtrSafe Function get_bool Lib "{INTERFACE}" ( _
    ByVal ptr_data As LongPtr) As Byte

Private Declare PtrSafe Function get_ptr_str Lib "{INTERFACE}" ( _
    ByVal PtrRustStr As LongPtr _
) As LongPtr

Private Declare PtrSafe Function init_array Lib "{INTERFACE}" ( _
    ByVal row As Long, _
    ByVal col As Long _
) As LongPtr

Private Declare PtrSafe Function arr_num_rows Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_num_cols Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i8 Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Byte, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i16 Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Integer, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i32 Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Long, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_i64 Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As LongLong, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_f32 Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Single, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_f64 Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Double, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_bool Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As Byte, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_none Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_str Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function arr_set_arr Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal val As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function get_elem_ptr Lib "{INTERFACE}" ( _
    ByVal ptr_arr As LongPtr, _
    ByVal row As Long, _
    ByVal col As Long, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function list_dll Lib "{INTERFACE}" ( _
    ByVal ptr_root As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function get_dll_note Lib "{INTERFACE}" ( _
    ByVal ptr_root As LongPtr, _
    ByVal ptr_dll_name As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function get_dll_args_info Lib "{INTERFACE}" ( _
    ByVal ptr_root As LongPtr, _
    ByVal ptr_dll_name As LongPtr, _
    ByVal ptr_result As LongPtr _
) As LongPtr

Private Declare PtrSafe Function call_dll Lib "{INTERFACE}" ( _
    ByVal ptr_root As LongPtr, _
    ByVal ptr_dll_name As LongPtr, _
    ByVal ptr_args As LongPtr, _
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
        
    ElseIf dtype = TYPE_ARRAY Then
        value = CreateArrayFromPointer(ptr_data)
        
    End If
    
    ReadPtrData = value

End Function

Private Function ConvertVBACollectionRustArr(ByVal vba_collection As Variant) As LongPtr

    Dim rust_arr_ptr As LongPtr
    Dim num_rows As Long
    Dim num_cols As Long
    Dim vba_arr() As Variant
    Dim set_result As Byte
    
    If TypeName(vba_collection) = "Range" Then
        vba_arr = vba_collection.value
        
    ElseIf IsArray(vba_collection) Then
        vba_arr = vba_collection
        
    Else
        ConvertVBACollectionRustArr = 0
        Exit Function
    
    End If
        
    num_rows = UBound(vba_arr, 1) - LBound(vba_arr, 1) + 1
    num_cols = UBound(vba_arr, 2) - LBound(vba_arr, 2) + 1
    
    rust_arr_ptr = init_array(num_rows, num_cols)
    
    Dim r As Long
    Dim c As Long
    Dim cur_row As Long
    Dim cur_col As Long
    
    cur_row = 0
    cur_col = 0

    For r = LBound(vba_arr, 1) To UBound(vba_arr, 1)
        
        cur_col = 0
        For c = LBound(vba_arr, 2) To UBound(vba_arr, 2)
        
            set_result = SetVBAValueToRustArr(rust_arr_ptr, cur_row, cur_col, vba_arr(r, c))
        
        cur_col = cur_col + 1
        Next c

        cur_row = cur_row + 1
    Next r
    
    ConvertVBACollectionRustArr = rust_arr_ptr

End Function

Private Function SetVBAValueToRustArr(ByVal ptr_rust_arr As LongPtr, ByVal row_idx As Long, ByVal col_idx, ByVal vb_val As Variant) As Byte

    Dim error As Byte
    Dim ptr_err As LongPtr
    Dim ptr_err_msg As LongPtr
    
    error = RUST_TRUE
    ptr_err = VarPtr(error)

    If VarType(vb_val) = vbEmpty Then
        ptr_err_msg = arr_set_none(ptr_rust_arr, row_idx, col_idx, ptr_err)
               
    ElseIf VarType(vb_val) = vbNull Then
        ptr_err_msg = arr_set_none(ptr_rust_arr, row_idx, col_idx, ptr_err)
    
    ElseIf VarType(vb_val) = vbByte Then
        ptr_err_msg = arr_set_i8(ptr_rust_arr, row_idx, col_idx, vb_val, ptr_err)
        
    ElseIf VarType(vb_val) = vbInteger Then
        ptr_err_msg = arr_set_i16(ptr_rust_arr, row_idx, col_idx, vb_val, ptr_err)
    
    ElseIf VarType(vb_val) = vbLong Then
        ptr_err_msg = arr_set_i32(ptr_rust_arr, row_idx, col_idx, vb_val, ptr_err)
    
    ElseIf VarType(vb_val) = vbLongLong Then
        ptr_err_msg = arr_set_i64(ptr_rust_arr, row_idx, col_idx, vb_val, ptr_err)
    
    ElseIf VarType(vb_val) = vbSingle Then
        ptr_err_msg = arr_set_f32(ptr_rust_arr, row_idx, col_idx, vb_val, ptr_err)
        
    ElseIf VarType(vb_val) = vbDouble Then
        ptr_err_msg = arr_set_f64(ptr_rust_arr, row_idx, col_idx, vb_val, ptr_err)
    
    ElseIf VarType(vb_val) = vbString Then
        ptr_err_msg = arr_set_str(ptr_rust_arr, row_idx, col_idx, StrPtr(vb_val), ptr_err)
    
    ElseIf VarType(vb_val) = vbBoolean Then
        If vb_val = True Then
            
            ptr_err_msg = arr_set_bool(ptr_rust_arr, row_idx, col_idx, RUST_TRUE, ptr_err)
        Else
            
            ptr_err_msg = arr_set_bool(ptr_rust_arr, row_idx, col_idx, RUST_FALSE, ptr_err)
        End If
        
    ElseIf VarType(vb_val) = vbArray Or TypeName(vb_val) = "Range" Or IsArray(vb_val) Then
        Dim converted_arr As LongPtr
        converted_arr = ConvertVBACollectionRustArr(vb_val)
        ptr_err_msg = arr_set_arr(ptr_rust_arr, row_idx, col_idx, converted_arr, ptr_err)
        
    Else
        ptr_err_msg = arr_set_none(ptr_rust_arr, row_idx, col_idx, ptr_err)
    
    End If
    
    If error = RUST_FALSE Then
        
        SetVBAValueToRustArr = RUST_FALSE
    Else
        
        SetVBAValueToRustArr = RUST_TRUE
    End If
        
End Function

Private Function IntoRustArgs(args() As Variant) As LongPtr
    
    Dim num_args As Long
    Dim arg As Variant
    Dim arg_idx As Long
    Dim set_rsult As Byte
    Dim ptr_rust_args As LongPtr
    
    num_args = UBound(args) - LBound(args) + 1
    arg_idx = 0
    set_result = RUST_TRUE
    ptr_rust_args = init_array(1, num_args)
    
    For Each arg In args
        set_result = SetVBAValueToRustArr(ptr_rust_args, 0, arg_idx, arg)
        
        If set_result = RUST_FALSE Then
            set_result = drop_data(ptr_rust_args)
            IntoRustArgs = 0
            Exit Function
        
        End If
        
        arg_idx = arg_idx + 1
        
    Next arg
    
    IntoRustArgs = ptr_rust_args
    
End Function

Function RustyFuncList() As Variant
    Dim ptr_dll_root As LongPtr
    Dim result As Byte
    Dim ptr_result As LongPtr
    Dim ptr_list As LongPtr

    ptr_dll_root = StrPtr(DLL_DIR_ROOT)
    result = RUST_TRUE
    ptr_result = VarPtr(result)

    ptr_list = list_dll(ptr_dll_root, ptr_result)

    If result = RUST_FALSE Then
        RustyFuncList = "Failed to get RustyFuncList"

    Else 
        RustyFuncList = ReadPtrData(ptr_list)
        drop_data(ptr_list)

    End If

End Function


Function RustyFuncNote(func_name As String) As Variant
    Dim ptr_dll_root As LongPtr
    Dim ptr_dll_name As LongPtr
    Dim result As Byte
    Dim ptr_result As LongPtr
    Dim ptr_note As LongPtr

    ptr_dll_root = StrPtr(DLL_DIR_ROOT)
    ptr_dll_name = StrPtr(func_name)
    result = RUST_TRUE
    ptr_result = VarPtr(result)

    ptr_note = get_dll_note(ptr_dll_root, ptr_dll_name, ptr_result)

    If result = RUST_FALSE Then
        RustyFuncNote = "Failed to get a note for the specified function"
    
    Else
        RustyFuncNote = ReadPtrData(ptr_note)
        drop_data(ptr_note)
    End If
    
End Function

Function RustyFuncArgs(func_name As String) As Variant
    Dim ptr_dll_root As LongPtr
    Dim ptr_dll_name As LongPtr
    Dim result As Byte
    Dim ptr_result As LongPtr
    Dim ptr_info As LongPtr

    ptr_dll_root = StrPtr(DLL_DIR_ROOT)
    ptr_dll_name = StrPtr(func_name)
    result = RUST_TRUE
    ptr_result = VarPtr(result)

    ptr_info = get_dll_args_info(ptr_dll_root, ptr_dll_name, ptr_result)

    If result = RUST_FALSE Then
        RustyFuncArgs = "Failed to get info for the specified function"
    
    Else    
        RustyFuncArgs = ReadPtrData(ptr_info)
        drop_data(ptr_info)
    
    End If

End Function


Function RustyFuncCall(func_name As String, ParamArray args() As Variant) As Variant
    Dim ptr_dll_root As LongPtr
    Dim ptr_dll_name As LongPtr
    Dim result As Byte
    Dim ptr_result As LongPtr
    Dim ptr_data As LongPtr

    Dim ptr_rust_args As LongPtr
    Dim result_from_rust As Variant
    Dim vba_args() As Variant
    
    Dim i As Long
    
    ReDim vba_args(LBound(args) To UBound(args))
    For i = LBound(args) To UBound(args)
        vba_args(i) = args(i)
    Next i
    
    ptr_rust_args = IntoRustArgs(vba_args)
    
    If ptr_rust_args = 0 Then
        RustyFuncCall = "Failed to prepare Rust args"
    
    Else

        result = RUST_TRUE

        ptr_result = VarPtr(result)
        ptr_dll_root = StrPtr(DLL_DIR_ROOT)
        ptr_dll_name = StrPtr(func_name)

        ptr_data = call_dll(ptr_dll_root, ptr_dll_name, ptr_rust_args, ptr_result)
        drop_result = drop_data(ptr_rust_args)
        
        RustyFuncCall = ReadPtrData(ptr_data)
        drop_result = drop_data(ptr_data)
        
    End If

End Function

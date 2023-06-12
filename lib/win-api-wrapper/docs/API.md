# win-api-wrapper

## timezoneapi.h

| Win API | safe-win-api |
| --- | --- |
| FileTimeToSystemTime | file_time_to_system_time |

## winbase.h

| Win API | safe-win-api |
| --- | --- |
| GetProcessIoCounters | process::get_io_counters |
| GetProcessAffinityMask | process::get_affinity_mask |
| SetProcessAffinityMask | process::set_affinity_mask |
| SetThreadAffinityMask | thread::set_affinity_mask |
| QueryFullProcessImageNameW | get_full_image_name |
|  | get_full_image_name_with_buffer |
| LocalFree | local_free |
| LocalHandle | get_local_handle |
| FormatMessageW | format_message |
|  | format_message_with_buffer |

## winuser.h

| Win API | safe-win-api |
| --- | --- |
| WaitForInputIdle | process::wait_for_input_idle |

## memoryapi.h

| Win API | safe-win-api |
| --- | --- |
| SetProcessWorkingSetSize | set_working_set_size |
|  | shrink_working_set |
| GetProcessWorkingSetSize | get_working_set_size |

## processthreadsapi.h

### process

| Win API | safe-win-api |
| --- | --- |
| SetProcessDefaultCpuSets | process::set_default_cpu_sets |
|  | process::clear_default_cpu_sets |
| SetProcessPriorityBoost | process::set_priority_boost |
| SetProcessAffinityUpdateMode | process::set_current_affinity_update_mode |
| SetPriorityClass | process::set_priority_class |
| GetProcessInformation | process::set_information |
| SetProcessPriorityBoost | process::get_information |
| TerminateProcess | process::terminate |
| GetProcessVersion | process::get_version |
| GetProcessTimes | process::get_times |
| GetPriorityClass | process::get_priority_class |
| GetProcessPriorityBoost | process::has_priority_boost |
| GetProcessIoCounters | process::get_io_counters |
| GetProcessId | process::get_id |
| IsProcessCritical | process::is_critical |
| GetProcessHandleCount | process::get_handle_count |
| GetExitCodeProcess | process::get_exit_code |
|  | process::is_running |
| GetProcessDefaultCpuSets | process::get_default_cpu_sets |
| OpenProcess | process::open |
| GetCurrentProcessId | process::get_current_id |
| GetCurrentProcess | process::get_current_handle |
| ExitProcess | process::exit_current |
| TerminateThread | thread::terminate |
| SwitchToThread | thread::switch_to_another |
| SetThreadInformation | thread::set_information |
| SetThreadPriorityBoost | thread::set_priority_boost |
| SetThreadPriority | thread::set_priority |
| SetThreadSelectedCpuSets | thread::set_selected_cpu_sets |
| | thread::clear_selected_cpu_sets |
| SetThreadIdealProcessorEx | thread::set_ideal_processor |
| GetThreadIdealProcessorEx | thread::get_ideal_processor |
| SetThreadDescription | thread::set_description |
| SuspendThread | thread::suspend |
| ResumeThread | thread::resume |
| GetThreadSelectedCpuSets | thread::get_selected_cpu_sets |
|  | thread::get_selected_cpu_set_count |
| GetThreadPriority | thread::has_priority_boost |
| GetThreadInformation | thread::get_information |
| GetThreadIOPendingFlag | thread::is_io_pending |
| GetThreadDescription | thread::get_description |
| GetProcessIdOfThread | thread::get_process_id |
| GetExitCodeThread | thread::get_exit_code |
|  | thread::is_running |
| GetCurrentThreadId | thread::get_current_id |
| GetCurrentThread | thread::get_current_handle |
| ExitThread | thread::exit_current |
| OpenThread | thread::open |


## processtopologyapi.h

| Win API | safe-win-api |
| --- | --- |
| GetProcessGroupAffinity | get_group_affinity |
|  | get_group_affinity_with_buffer |

## securitybaseapi.h

| Win API | safe-win-api |
| --- | --- |
| GetTokenInformation | get_token_information |

## handleapi.h

| Win API | safe-win-api |
| --- | --- |
| CloseHandle | close_handle |

## errhandlingapi.h

| Win API | safe-win-api |
| --- | --- |
| GetLastError | get_last_error |

## psapi.h

| Win API | safe-win-api |
| --- | --- |
| EnumProcesses | get_pids |
|  | get_pids_with_buffer |

## tlhelp32

| Win API | safe-win-api |
| --- | --- |
| CreateToolhelp32Snapshot | create_snapshot |
| Process32FirstW | first_process |
| Process32NextW | next_process |
| Thread32First | first_thread |
| Thread32Next | next_thread |

## combined

| safe-win-api |
| --- |
| is_elevated |
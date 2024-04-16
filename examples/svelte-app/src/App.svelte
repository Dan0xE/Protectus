<script lang="ts">
  import {checkIfDebuggerPresent, checkIfProtected, checkIfVirtualMachine, getHardwareId, getSerialNumberData, getSerialNumberState} from 'protectus'
  import {message} from '@tauri-apps/plugin-dialog'

  type SomeFunc<T> = (...args: T[]) => Promise<T>;

  function _message(msg: string, kind: "warning" | "error", title: string) {
    message(msg, {
      kind,
      title
    })
  }


  function _executer<T>(name: string, ...func: SomeFunc<T>[]) {
    func.forEach((fn => fn().then((res) => {
      console.log(res)
      
      if(res === "myhwid") {
        return _message("Cannot access HWID, please protect the Application with VmProtect first", "error", "HWID Check")
      }

      let result = typeof res === "object" ? JSON.stringify(res) : res
      _message(`Result is: ${result}`, "warning", name)}).catch((e) => _message(`Something went wrong: ${e}`, "error", name))))
  }

  const _libfuncs = [
    {
      "name": "Check if Debugger Present (Non Kernel Level)",
      "functionToCall": checkIfDebuggerPresent(false),
    },
    {
      "name": "Check if protected",
      "functionToCall": checkIfProtected(),
    },
    {
      "name": "Check if Debugger Present (Kernel Level)",
      "functionToCall": checkIfDebuggerPresent(true),
    },
    {
      "name": "Check if running in Virtual Machine",
      "functionToCall": checkIfVirtualMachine(),
    },
    {
      "name": "Get Hardware ID",
      "functionToCall": getHardwareId()
    },
    {
      "name": "Get Serial Number Data",
      "functionToCall": getSerialNumberData(),
    },
    {
      "name": "Get Serial Number State",
      "functionToCall": getSerialNumberState()
    }
  ]
</script>

<main class="container">
  <h1>Protectus Demo</h1>

  <div class="col">
   {#each _libfuncs as func}
    <button on:click={() => _executer(func.name, () => func.functionToCall)}>
    {func.name}
   </button>
   {/each}
  </div>
</main>
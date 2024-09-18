using System.Management.Automation;
using System.Collections.ObjectModel;

namespace ProtoHandler {
    public class Runner {
        private PowerShell ps;
        private bool initialized = false;

        public Runner() {
            ps = PowerShell.Create();
        }

        public Runner AddScript(string script) {
            if (File.Exists(script)) {
                string scriptContent = File.ReadAllText(script);
                ps.AddScript(scriptContent);
            } else {
                ps.AddScript(script);
            }
            initialized = true;
            return this;
        }

        public bool isInitialized() {
            return initialized;
        }

        public Runner AddParameter(string name, object value) {
            ps.AddParameter(name, value);
            return this;
        }

        public Collection<PSObject> Run() {
            return ps.Invoke();
        }
    }
}

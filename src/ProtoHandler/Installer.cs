
namespace ProtoHandler {
    class Installer {
        private static string defaultPath = @"%LOCALAPPDATA\protoHandler";
        private static string defaultProto = @"snip-proto";
        public string? Path { get; set; }
        public string? Protocol { get; set; }

        public Installer() {
            Path = Environment.ExpandEnvironmentVariables(defaultPath);
            Protocol = defaultProto;
        }

        public Installer(string path, string proto) {
            Path = path;
            Protocol = proto;
        }

        private void SetRegistry() { }

        private void WriteSettings() { }

        public void Run() {
            SetRegistry();
            WriteSettings();
        }

    }

}

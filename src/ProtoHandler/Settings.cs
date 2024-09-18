using Microsoft.CodeAnalysis.CSharp.Syntax;
using Microsoft.Extensions.Configuration;

namespace ProtoHandler {
    public class Settings {
        private static string baseDirectory = AppDomain.CurrentDomain.BaseDirectory;
        private static string defaultLogFile = Path.Join(baseDirectory, "logs", "protohandler.log");
        private static string defaultScriptPath = Path.Join(baseDirectory, "scripts", "no-op.ps1");

        // TODO: For now just look in the same directory, but maybe it should be in ~/.config
        private static string defaultSettingsFile = Path.Join(baseDirectory, "protohandler.json");


        public string LogFile { get; set; } = defaultLogFile;

        public string ScriptPath { get; set; } = defaultScriptPath;

        public Settings() {
            // TODO: This seems redundant since the members are set with the same values
            LogFile = defaultLogFile;
            ScriptPath = defaultScriptPath;
        }

        public Settings(string path) {
            this.Load(path);
        }

        public void Load(string? path = "") {
            if (String.IsNullOrEmpty(path))
                path = defaultSettingsFile;

            if (!File.Exists(path))
                throw new FileNotFoundException($"Could not find file '{path}'");

            var builder = new ConfigurationBuilder()
                .AddJsonFile(path, optional: false, reloadOnChange: true);

            IConfigurationRoot root = builder.Build();

            if (!String.IsNullOrEmpty(root["LogFile"])) {
                this.LogFile = root["LogFile"] ?? defaultLogFile;
            }

            if (!String.IsNullOrEmpty(root["ScriptPath"])) {
                this.ScriptPath = root["ScriptPath"] ?? defaultScriptPath;
            }
        }
    }
}

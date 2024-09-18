using System.ComponentModel;
using CommandLineParser.Arguments;
using CommandLineParser.Validation;

namespace ProtoHandler {

    [DistinctGroupsCertification("u,l,s", "i,p")]
   class ArgumentParser {

        [ValueArgument(typeof(string), 'u', "uri", Description = "The URI of the ")]
        public string? Uri { get; set; } = "";

        [SwitchArgument('i', "install", true, Description = "Install a protocol handler")]
        public bool Install = false;

        [ValueArgument(typeof(string), 'p', "protocol", Description = "The name of the protocol")]
        public string? Protocol { get; set; } = "";

        [ValueArgument(typeof(string), 's', "settings", Description = "The settings file to use")]
        public string? Settings { get; set; } = "";

        [ValueArgument(typeof(string), 'l', "log", Description = "The file to log output to")]
        public string? LogFile { get; set; } = "";
   }
}

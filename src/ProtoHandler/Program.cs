using System.Web;
using System.Collections.ObjectModel;
using System.Management.Automation;
using CommandLineParser.Exceptions;

namespace ProtoHandler {
    class Program {
        static void Main(string[] args) {
            CommandLineParser.CommandLineParser parser = new CommandLineParser.CommandLineParser();
            ArgumentParser p = new ArgumentParser();
            Settings config = new Settings();


            parser.ExtractArgumentAttributes(p);
            try {
                parser.ParseCommandLine(args);
            }
            catch (CommandLineException e) {
                Console.WriteLine(e.Message);
                parser.ShowUsage();
            }

            if (!String.IsNullOrEmpty(p.Settings)) {
                if (File.Exists(p.Settings))
                    config.Load(p.Settings);
                else {
                    throw new FileNotFoundException($"Settings file {p.Settings} could not be found");
                }
            } else {
                // Load the default file
                config.Load();
            }

            if (!String.IsNullOrEmpty(p.LogFile)) {
                if (!File.Exists(p.LogFile))
                    throw new FileNotFoundException($"Log file {p.LogFile} not found");
                else {
                    config.LogFile = p.LogFile;
                }
            }



            Log log = new Log(config.LogFile);
            Runner runner = new Runner();

            log.Write(String.Format(
                @"Starting ProtocolHandler in {0}",
                AppDomain.CurrentDomain.BaseDirectory));


            if (!File.Exists(config.ScriptPath))
                log.Write(String.Format(
                    @"ERROR could not find scriptPath {0}",
                config.ScriptPath));
            else
                log.Write(String.Format(
                    @"Loading script from Path {0}",
                config.ScriptPath));

            string decodedUri = HttpUtility.UrlDecode(args[0]);

            try {
                log.Write(String.Format(
                    @"- setting URI to {0}",
                decodedUri));
                runner
                    .AddScript(config.ScriptPath)
                    .AddParameter(@"Uri", decodedUri)
                    .AddParameter(@"LogFile", config.LogFile);

                log.Write(@"- Invoking script");
                Collection<PSObject> results = runner.Run();
                foreach (PSObject result in results) {
                    log.Write(String.Format(
                            @"- Script output: '{0}'", result));
                }
            }
            catch (Exception e) {
                log.Write(e.Message);
            }
            log.Write(@"protoHandler finished");
        }
    }
}

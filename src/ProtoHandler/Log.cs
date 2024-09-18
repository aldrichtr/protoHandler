using System.IO;
using System.Management.Automation;
using Microsoft.PowerShell.Commands;

namespace ProtoHandler {
	public class Log {
		private static string prefix = $"[{DateTime.Now}]";

		public string? FilePath { get; set; }

		public void Write(string msg) {
			if (String.IsNullOrEmpty(FilePath))
				throw new ApplicationException("The file path for logging has not been set");
			using (StreamWriter sw = File.AppendText(FilePath)) {
				sw.WriteLine($"{prefix}: {msg}");
			}
		}

		public Log() { }

		public Log(string path) {
			if (File.Exists(path))
				FilePath = path;
			else
				createLog(path);
		}

		private void createLog(string? path) {
			// if we were given a path, then set it as the FilePath
			if (!String.IsNullOrEmpty(path))
				FilePath = path;

			// Throw only if no `path` ws given and FilePath was not set earlier
			if (String.IsNullOrEmpty(FilePath))
				throw new ApplicationException("The file path for logging has not been set");


			string logDirectory = Path.GetDirectoryName(FilePath) ?? string.Empty;

			if (!Directory.Exists(logDirectory))
				Directory.CreateDirectory(logDirectory);

			if (!File.Exists(FilePath))
				using (StreamWriter sw = File.CreateText(FilePath)) {
					sw.WriteLine($"--- Log file started {DateTime.Now} ---");
				}
		}

	}
}

import hashlib
import unittest
from unittest.mock import MagicMock, patch

import update_password


class ParameterNotFound(Exception):
    pass


class UpdatePasswordTests(unittest.TestCase):
    def setUp(self):
        self.ssm = MagicMock()
        self.ssm.exceptions.ParameterNotFound = ParameterNotFound
        self.patcher = patch.object(update_password, "ssm", self.ssm)
        self.patcher.start()
        self.addCleanup(self.patcher.stop)

    def test_updates_only_existing_password_hash(self):
        update_password.update_password("alice", "new password")

        self.ssm.get_parameter.assert_called_once_with(
            Name="/secrets/alice/password_hash"
        )
        self.ssm.put_parameter.assert_called_once_with(
            Name="/secrets/alice/password_hash",
            Value=hashlib.sha256(b"new password").hexdigest(),
            Type="SecureString",
            Overwrite=True,
        )

    def test_missing_user_is_not_created(self):
        self.ssm.get_parameter.side_effect = ParameterNotFound()

        with self.assertRaisesRegex(ValueError, "not found"):
            update_password.update_password("missing", "new password")

        self.ssm.put_parameter.assert_not_called()

    def test_normalizes_username_like_existing_tools(self):
        update_password.update_password("ssm user", "new password")

        self.ssm.get_parameter.assert_called_once_with(
            Name="/secrets/user_ssm_user/password_hash"
        )

    @patch("update_password.getpass.getpass", side_effect=["first", "second"])
    @patch("sys.argv", ["update_password.py", "alice"])
    def test_mismatched_confirmation_does_not_update(self, _getpass):
        with self.assertRaises(SystemExit) as result:
            update_password.main()

        self.assertEqual(result.exception.code, 1)
        self.ssm.get_parameter.assert_not_called()
        self.ssm.put_parameter.assert_not_called()


if __name__ == "__main__":
    unittest.main()